//! # Game runtime

use super::{entity::Enemy, session::Action, GameResult, Options, Session};
use crate::audio::{AudioEngine, Sound, Theme};
use crate::gfx::{ascii_art, Render, Room as RoomToRender};
use crate::ui::{GameMsg, Id, LoadGameMsg, MenuId, MenuMsg, Msg, Ui};
use crate::utils::saved_games::SavedGameFiles;

use std::path::{Path, PathBuf};
use tuirealm::props::{Color, Shape};

/// Game runtime
pub struct Runtime {
    audio: Option<AudioEngine>,
    saved_games_dir: PathBuf,
    render: Render,
    running: bool,
    session: Option<Session>,
    ui: Ui,
}

impl Runtime {
    /// Setup game runtime
    pub fn setup(options: Options) -> GameResult<Self> {
        debug!("setting up game runtime");
        let audio = if options.muted {
            None
        } else {
            debug!("configuring audio engine");
            AudioEngine::new(Theme::None).map(|x| Some(x))?
        };
        info!("audio OK");
        debug!("initializing Ui");
        let mut ui = Ui::new()?;
        info!("Ui OK");
        let (width, height) = ui.sizes()?;
        let render = Render::new(width, height);
        info!("Render engine configured to work on {}x{}", width, height);
        // loading menu
        debug!("loading menu");
        ui.load_menu()?;
        info!("menu loaded");
        Ok(Self {
            audio,
            saved_games_dir: options.saved_games_dir,
            render,
            running: true,
            session: None,
            ui,
        })
    }

    /// Run game
    pub fn run(mut self) -> GameResult<()> {
        debug!("initializing terminal...");
        self.ui.init_terminal();
        debug!("playing Menu theme...");
        self.play_theme(Theme::Menu)?;
        while self.running {
            // Run view
            let mut redraw = true;
            let messages = self.ui.tick()?;
            for msg in messages.into_iter() {
                self.update(msg)?;
                redraw = true;
            }
            if redraw {
                self.ui.view()?;
            }
        }
        // Finalize terminal and stop sound
        debug!("finalizing terminal...");
        self.ui.finalize_terminal();
        debug!("stopping theme...");
        self.play_theme(Theme::None)?;

        Ok(())
    }

    /// Play sound
    fn play_sound(&mut self, sound: Sound) {
        if let Some(audio) = self.audio.as_mut() {
            audio.play(sound.track());
        }
    }

    /// Play theme
    fn play_theme(&mut self, theme: Theme) -> GameResult<()> {
        if let Some(audio) = self.audio.as_mut() {
            audio.play_theme(theme)?;
        }
        Ok(())
    }

    /// Start gameplay in the maze
    fn start_maze(&mut self, session: Session) -> GameResult<()> {
        self.session = Some(session);
        self.play_theme(Theme::Maze)?;
        self.ui.load_game()?;

        Ok(())
    }

    /// load saved game
    fn load_game(&mut self, game_file: &Path) -> GameResult<()> {
        debug!("loading game {}", game_file.display());
        let session = match SavedGameFiles::load_game(game_file) {
            Ok(s) if s.is_version_compatible() => s,
            Ok(_) => {
                self.ui.show_load_game_error("incompatible game version")?;
                return Ok(());
            }
            Err(e) => {
                self.ui
                    .show_load_game_error(format!("failed to load game: {}", e))?;
                return Ok(());
            }
        };
        self.start_maze(session)
    }

    /// Save game as name
    fn save_game(&mut self, name: &str) -> GameResult<()> {
        if let Some(session) = self.session.as_ref() {
            debug!("saving game as {}", name);
            SavedGameFiles::save_game(name, &self.saved_games_dir, &session)?;
        }
        Ok(())
    }

    /// Play action in game
    fn play_action(&mut self, action: Action) -> GameResult<()> {
        debug!("playing action {:?}", action);
        let effect = self.session.as_mut().unwrap().play_turn(action);
        // play sounds
        for sound in effect.sounds {
            self.play_sound(sound);
        }
        // show messages
        debug!("updating messages: {:?}", effect.messages);
        self.ui
            .update_game_messages(&effect.messages, self.session.as_ref().unwrap())?;
        debug!("updating player health");
        self.ui
            .update_game_player_health(self.session.as_ref().unwrap().player().health())?;
        let fighting_enemy = self.session.as_ref().unwrap().get_fighting_enemy();
        if let Some(enemy) = fighting_enemy {
            debug!("updating enemy data: {:?}", enemy);
            self.ui
                .update_game_enemy_data(enemy.health(), enemy.name())?;
        } else {
            debug!("hiding enemy data");
            self.ui.hide_game_enemy_data()?;
        }
        // change theme if state changed
        self.switch_maze_theme()?;
        if self.session.as_ref().unwrap().game_over() {
            info!("player is dead; show game over");
            self.ui.show_game_gameover_popup()?;
            self.play_theme(Theme::GameOver)?;
        }
        if self.session.as_ref().unwrap().has_won() {
            info!("player has won; show victory");
            self.ui.load_victory()?;
            self.play_theme(Theme::Victory)?;
        }
        // update actions
        let possible_actions = self.session.as_ref().unwrap().available_actions();
        debug!("updating possible actions: {:?}", possible_actions);
        self.ui
            .update_game_actions(possible_actions, self.session.as_ref().unwrap())?;
        // update canvas
        self.render_shapes()?;
        Ok(())
    }

    /// render shapes in canvas
    fn render_shapes(&mut self) -> GameResult<()> {
        debug!("rendering shapes");
        let room = self.render.render_room(self.room_to_render());
        debug!("room rendered");
        let entity = if let Some(enemy) = self.session.as_ref().unwrap().get_fighting_enemy() {
            debug!("rendering enemy {:?}", enemy);
            self.render_enemy(enemy)?
        } else if let Some(item) = self.session.as_ref().unwrap().get_item_in_the_room() {
            debug!("rendering item {:?}", item);
            self.render
                .ascii_art(0.0, 0.0, ascii_art::CHEST, Color::Yellow)
        } else {
            vec![]
        };
        let shapes = self.render.stack(vec![room, entity]);
        self.ui.update_game_canvas(&shapes)?;
        Ok(())
    }

    fn room_to_render(&self) -> RoomToRender {
        todo!();
    }

    fn render_enemy(&self, enemy: &Enemy) -> GameResult<Vec<Shape>> {
        todo!()
    }

    fn switch_maze_theme(&mut self) -> GameResult<()> {
        if self
            .session
            .as_ref()
            .unwrap()
            .get_fighting_enemy()
            .is_some()
            && self
                .audio
                .as_ref()
                .map(|x| x.theme())
                .unwrap_or(Theme::Fight)
                != Theme::Fight
        {
            self.play_theme(Theme::Fight)?;
        } else if self
            .session
            .as_ref()
            .unwrap()
            .get_fighting_enemy()
            .is_none()
            && self
                .audio
                .as_ref()
                .map(|x| x.theme())
                .unwrap_or(Theme::Maze)
                != Theme::Maze
        {
            self.play_theme(Theme::Maze)?;
        }
        Ok(())
    }

    /// Custom implementation of the Update trait
    fn update(&mut self, msg: Msg) -> GameResult<()> {
        match msg {
            Msg::None => Ok(()),
            Msg::Game(msg) => self.update_game(msg),
            Msg::LoadGame(msg) => self.update_load_game(msg),
            Msg::Menu(msg) => self.update_menu(msg),
        }
    }

    fn update_game(&mut self, msg: GameMsg) -> GameResult<()> {
        match msg {
            GameMsg::ActionSelected(action) => {
                self.play_action(action)?;
                self.play_sound(Sound::Input);
            }
            GameMsg::CloseErrorPopup => {
                self.ui.close_game_error_popup()?;
                self.play_sound(Sound::Input);
            }
            GameMsg::CloseInventory => {
                self.ui.close_game_inventory()?;
                self.play_sound(Sound::Input);
            }
            GameMsg::CloseQuitPopup => {
                self.ui.close_game_quit_popup()?;
                self.play_sound(Sound::Input);
            }
            GameMsg::CloseSaveFileName => {
                self.ui.close_game_save_file_name()?;
                self.play_sound(Sound::Input);
            }
            GameMsg::GameOver => {
                info!("game over; destroy session and show game over");
                self.session = None;
                self.ui.load_game_over()?;
            }
            GameMsg::Quit(save) => {
                self.play_sound(Sound::Input);
                if save {
                    if let Err(err) = self.save_game("autosave") {
                        error!("failed to save game: {}", err);
                        self.ui
                            .show_game_error_popup(format!("failed to save game: {}", err))?;
                        return Ok(());
                    }
                }
                self.session = None;
                self.ui.load_menu()?;
            }
            GameMsg::SaveGame(name) => {
                self.ui.close_game_save_file_name()?;
                self.play_sound(Sound::Input);
                if let Err(err) = self.save_game(&name) {
                    error!("failed to save game: {}", err);
                    self.ui
                        .show_game_error_popup(format!("failed to save game: {}", err))?;
                }
            }
            GameMsg::ShowInventory => {
                self.play_sound(Sound::Input);
                if let Some(session) = self.session.as_ref() {
                    self.ui.show_game_inventory(session)?;
                }
            }
            GameMsg::ShowQuitPopup => {
                self.play_sound(Sound::Input);
                self.ui.show_game_quit_popup()?;
            }
            GameMsg::ShowSaveFileName => {
                self.play_sound(Sound::Input);
                self.ui.show_game_save_file_name()?;
            }
            GameMsg::UseItem(item) => {
                self.play_sound(Sound::Input);
                self.play_action(Action::UseItem(item))?;
            }
        }
        Ok(())
    }

    fn update_load_game(&mut self, msg: LoadGameMsg) -> GameResult<()> {
        match msg {
            LoadGameMsg::CloseErrorPopup => {
                self.play_sound(Sound::Input);
                self.ui.close_load_game_error()?;
            }
            LoadGameMsg::GoToMenu => {
                self.play_sound(Sound::Input);
                self.ui.load_menu()?;
            }
            LoadGameMsg::GameChanged(p) => match SavedGameFiles::load_game(&p) {
                Err(e) => {
                    self.play_sound(Sound::Input);
                    self.ui
                        .show_load_game_error(format!("failed to load game: {}", e))?;
                }
                Ok(session) => {
                    self.play_sound(Sound::Input);
                    self.ui.set_load_game_save_metadata(&session)?;
                }
            },
            LoadGameMsg::LoadGame(game_file) => {
                self.play_sound(Sound::Input);
                self.load_game(&game_file)?;
            }
        }
        Ok(())
    }

    fn update_menu(&mut self, msg: MenuMsg) -> GameResult<()> {
        match msg {
            MenuMsg::ActiveExit => {
                self.play_sound(Sound::Input);
                self.ui.active(Id::Menu(MenuId::Exit));
            }
            MenuMsg::ActiveLoadGame => {
                self.play_sound(Sound::Input);
                self.ui.active(Id::Menu(MenuId::LoadGame));
            }
            MenuMsg::ActiveNewGame => {
                self.play_sound(Sound::Input);
                self.ui.active(Id::Menu(MenuId::NewGame));
            }
            MenuMsg::ActiveSeed => {
                self.play_sound(Sound::Input);
                self.ui.active(Id::Menu(MenuId::Seed));
            }
            MenuMsg::LoadGame => {
                let saved_games = SavedGameFiles::saved_games(&self.saved_games_dir)?;
                if saved_games.is_empty() {
                    self.play_sound(Sound::Error);
                    self.play_sound(Sound::Input);
                } else {
                    let game_0 = match saved_games.get(0) {
                        None => None,
                        Some(p) => SavedGameFiles::load_game(p).ok(),
                    };
                    self.ui.load_game_loader(&saved_games, game_0.as_ref())?;
                }
            }
            MenuMsg::NewGame => {
                self.play_sound(Sound::Input);
                // create a new session
                let seed = self.ui.get_menu_seed()?;
                debug!("initializing new session with seed {:?}", seed);
                self.start_maze(Session::new(seed))?;
            }
            MenuMsg::Quit => {
                self.play_sound(Sound::Input);
                self.play_theme(Theme::None)?;
                self.running = false;
            }
        }
        Ok(())
    }
}
