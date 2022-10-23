//! # Room graffitis

pub fn graffiti(room: u32) -> &'static str {
    GRAFFITIS[room as usize % GRAFFITIS.len()]
}

const GRAFFITIS: &[&'static str] = &[
    GRAFFITI_A, GRAFFITI_B, GRAFFITI_C, GRAFFITI_D, GRAFFITI_E, GRAFFITI_F, GRAFFITI_G, GRAFFITI_H,
    GRAFFITI_I, GRAFFITI_J, GRAFFITI_K, GRAFFITI_L,
];

const GRAFFITI_A: &str = r#"*   *
 * *
  *
 * *
*   *"#;

const GRAFFITI_B: &str = r#"* * *
* * *
 * * *
 * * *
  * * *
  * * *"#;

const GRAFFITI_C: &str = r#"     *
    * *
   *   *
  *     *
  *     *
   *   *
    ***"#;

const GRAFFITI_D: &str = r#"    *
    *
   *** 
  *   *
***   ***
  *   *
   *** 
    *
    *"#;

const GRAFFITI_E: &str = r#"  ***
 *    
 ***  
*   * 
*   * 
*   * 
 ***  "#;

const GRAFFITI_F: &str = r#"  *****
 *     *
 * * * *
 *     *
  *****  "#;

const GRAFFITI_G: &str = r#"  *
 ***
*****
 ***
  *"#;

const GRAFFITI_H: &str = r#"   *
   *
   * 
   * 
*******
   * 
   * "#;

const GRAFFITI_I: &str = r#"  *
*****
 ***
*   *"#;

const GRAFFITI_J: &str = r#" ****
******    *
***      * *
******    *
 ****"#;

const GRAFFITI_K: &str = r#""#;

const GRAFFITI_L: &str = r#""#;
