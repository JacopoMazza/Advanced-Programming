#[derive(Debug)]
pub struct Sentence{
    pub words: Vec<String>,
}

impl Sentence {
    pub fn new_default() -> Sentence {
        Sentence {
            words: Vec::new()
        }
    }

    pub fn new(s: &str) -> Sentence {

        let words =s.split(" ").collect::<Vec<&str>>();
        Sentence{
            words:words.iter().map(|w| w.to_string()).collect()
        }
    }


}


pub mod test {
    use std::collections::HashMap;
    use crate::sentence::sentence::Sentence;

    pub fn magic_sentence(s_list: &HashMap<i32, Sentence>, i: i32, j: i32) -> Result<crate::sentence::sentence::Sentence,&'static str>{
        match   (s_list.get(&i), s_list.get(&j)){
            (Some(s1), Some(s2)) => {
                /*let mut common = Vec::new();
                for (w1, w2) in s1.words.iter().zip(s2.words.iter()){
                    if w1 == w2{
                        common.push(w1);
                    }
                }
                Ok(Sentence{
                    words: common.iter().map(|w| w.to_string()).collect()
                })
*/

                let s = s1.words.iter().zip(s2.words.iter()).map(|(w1,w2)| {
                    if w1 == w2 {String::from(w1)}
                    else {"".to_string()}
                }).filter(|w| w != "").collect();

                return Ok(crate::sentence::sentence::Sentence{words:s});

            }

            (_, _) => Err("sentence not found")
        }
    }

}