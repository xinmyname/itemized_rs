use crate::models;

pub struct ItemFactory {}

impl ItemFactory {

    pub fn new() -> ItemFactory {
        ItemFactory { }
    }

    pub fn make_item(&self) -> models::Item {
        models::Item::new()
    }
}

use regex::Regex;
use std::sync::Mutex;

pub struct Pluralizer {
    uncountables: Vec<&'static str>,
    rules: Vec<(&'static str, &'static str)>
}

impl Pluralizer {
    pub fn new() -> Pluralizer {
        return Pluralizer {
            uncountables: vec!(
                "access", "accommodation", "adulthood", "advertising", "advice", "aggression",
                "aid", "air", "alcohol", "anger", "applause", "arithmetic", "art",
                "assistance", "athletics", "attention", "bacon", "baggage", "ballet",
                "beauty", "beef", "beer", "biology", "bison", "botany", "bread", "butter",
                "carbon", "cash", "chaos", "cheese", "chess", "childhood", "clothing",
                "coal", "coffee", "commerce", "compassion", "comprehension", "content",
                "corps", "corruption", "cotton", "courage", "cream", "currency", "dancing",
                "danger", "data", "deer", "delight", "dignity", "dirt", "distribution",
                "dust", "economics", "education", "electricity", "employment", "engineering",
                "envy", "equipment", "ethics", "evidence", "evolution", "faith", "fame",
                "fish", "flour", "flu", "food", "freedom", "fuel", "fun", "furniture",
                "garbage", "garlic", "genetics", "gold", "golf", "gossip", "grammar",
                "gratitude", "grief", "ground", "guilt", "gymnastics", "hair", "happiness",
                "hardware", "harm", "hate", "hatred", "health", "heat", "height", "help",
                "homework", "honesty", "honey", "hospitality", "housework", "humour",
                "hunger", "hydrogen", "ice", "importance", "inflation", "information",
                "injustice", "innocence", "iron", "irony", "jealousy", "jelly", "judo",
                "karate", "kindness", "knowledge", "labour", "lack", "laughter", "lava",
                "leather", "leisure", "lightning", "linguistics", "litter", "livestock",
                "logic", "loneliness", "luck", "luggage", "machinery", "magic", "management",
                "mankind", "marble", "mathematics", "mayonnaise", "means", "measles", "meat",
                "methane", "milk", "money", "moose", "mud", "music", "nature", "news",
                "nitrogen", "nonsense", "nurture", "nutrition", "obedience", "obesity",
                "oil", "oxygen", "passion", "pasta", "patience", "permission", "physics",
                "poetry", "pollution", "poverty", "power", "pronunciation", "psychology",
                "publicity", "quartz", "racism", "rain", "relaxation", "reliability",
                "research", "respect", "revenge", "rice", "rubbish", "rum", "salad",
                "satire", "scissors", "seaside", "series", "shame", "sheep", "shopping",
                "silence", "sleep", "smoke", "smoking", "snow", "soap", "software", "soil",
                "sorrow", "soup", "species", "speed", "spelling", "steam", "stuff",
                "stupidity", "sunshine", "swine", "symmetry", "tennis", "thirst", "thunder",
                "toast", "tolerance", "toys", "traffic", "transporation", "travel", "trust",
                "understanding", "unemployment", "unity", "validity", "veal", "vengeance",
                "violence"),
            rules: vec!(
                ("(th)is$", "${1}ese"),
                ("(th)at$", "${1}ose"),
                ("(millen)ium$", "${1}ia"),
                ("(l)eaf$", "${1}eaves"),
                ("(r)oof$", "${1}oofs"),
                ("(gen)us$", "${1}era"),
                ("(embarg)o$", "${1}oes"),
                ("arf$", "arves"),
                ("^(b|tabl)eau$", "${1}eaux"),
                ("^(append|matr)ix$", "${1}ices"),
                ("^(ind)ex$", "${1}ices"),
                ("^(a)pparatus$", "${1}pparatuses"),
                ("^(a)lumna$", "${1}lumnae"),
                ("^(alg|vertebr|vit)a$", "${1}ae"),
                ("^(d)ie$", "${1}ice"),
                ("(m|l)ouse$", "${1}ice"),
                ("^(p)erson$", "${1}eople"),
                ("^(o)x$", "${1}xen"),
                ("^(c)hild$", "${1}hildren"),
                ("(g)oose$", "${1}eese"),
                ("(t)ooth$", "${1}eeth"),
                ("lf$", "lves"),
                ("(f)oot$", "${1}eet"),
                ("^(wo|work|fire)man$", "${1}men"),
                ("(potat|tomat|volcan)o$", "${1}oes"),
                ("(criteri|phenomen)on$", "${1}a"),
                ("(nebul)a", "${1}ae"),
                ("oof$", "ooves"),
                ("ium$", "ia"),
                ("um$", "a"),
                ("oaf$", "oaves"),
                ("(thie)f$", "${1}ves"),
                ("fe$", "ves"),
                ("(buffal|carg|mosquit|torped|zer|vet|her|ech)o$", "${1}oes"),
                ("o$", "os"),
                ("ch$", "ches"),
                ("sis$", "ses"),
                ("(corp)us$", "${1}ora"),
                ("(cact|nucle|alumn|bacill|fung|radi|stimul|syllab)us$", "${1}i"),
                ("(ax)is", "${1}es"),
                ("(sh|zz|ss)$", "${1}es"),
                ("x$", "xes"),
                ("(t|sp|r|l|b)y$", "${1}ies"),
                ("s$", "ses"),
                ("$", "s"))
        }
    }

    pub fn plural_of<S:Into<String>>(&self, word_:S, count:i32) -> String {

        let word: String = word_.into();

        if count == 1 || word.is_empty() {
            return word;
        }

        let uncountable = match self.uncountables.binary_search(&word.as_ref()) {
            Ok(_) => true, _ => false
        };

        if uncountable {
            return word;
        }

        for &(rule, template) in &self.rules {
            
            let regex = Regex::new(rule).unwrap();
            let result = regex.replace(&word, template);

            if result != word {
                return result.to_string();
            }
        }

        return word;
    }
}

lazy_static! {
    static ref DEFAULT: Mutex<Pluralizer> = Mutex::new(Pluralizer::new());
}

pub fn plural_of<S:Into<String>>(word:S, count:i32) -> String {
    return DEFAULT.lock().unwrap().plural_of(word, count);
}

#[cfg(test)]
mod the_plural_of {
    use super::*;

    #[test]
    fn cat_is_cats() {
        assert_eq!(plural_of("cat", 2), "cats");
    }
}
