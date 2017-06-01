use regex::Regex;

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
                ("(th)is$", "$1ese"),
                ("(th)at$", "$1ose"),
                ("(millen)ium$", "$1ia"),
                ("(l)eaf$", "$1eaves"),
                ("(r)oof$", "$1oofs"),
                ("(gen)us$", "$1era"),
                ("(embarg)o$", "$1oes"),
                ("arf$", "arves"),
                ("^(b|tabl)eau$", "$1eaux"),
                ("^(append|matr)ix$", "$1ices"),
                ("^(ind)ex$", "$1ices"),
                ("^(a)pparatus$", "$1pparatuses"),
                ("^(a)lumna$", "$1lumnae"),
                ("^(alg|vertebr|vit)a$", "$1ae"),
                ("^(d)ie$", "$1ice"),
                ("(m|l)ouse$", "$1ice"),
                ("^(p)erson$", "$1eople"),
                ("^(o)x$", "$1xen"),
                ("^(c)hild$", "$1hildren"),
                ("(g)oose$", "$1eese"),
                ("(t)ooth$", "$1eeth"),
                ("lf$", "lves"),
                ("(f)oot$", "$1eet"),
                ("^(wo|work|fire)man$", "$1men"),
                ("(potat|tomat|volcan)o$", "$1oes"),
                ("(criteri|phenomen)on$", "$1a"),
                ("(nebul)a", "$1ae"),
                ("oof$", "ooves"),
                ("ium$", "ia"),
                ("um$", "a"),
                ("oaf$", "oaves"),
                ("(thie)f$", "$1ves"),
                ("fe$", "ves"),
                ("(buffal|carg|mosquit|torped|zer|vet|her|ech)o$", "$1oes"),
                ("o$", "os"),
                ("ch$", "ches"),
                ("sis$", "ses"),
                ("(corp)us$", "$1ora"),
                ("(cact|nucle|alumn|bacill|fung|radi|stimul|syllab)us$", "$1i"),
                ("(ax)is", "$1es"),
                ("(sh|zz|ss)$", "$1es"),
                ("x$", "xes"),
                ("(t|r|l|b)y$", "$1ies"),
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
