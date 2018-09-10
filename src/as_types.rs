
use serde_json;
use std::collections::HashMap;

/// Hold Zero-To-Many Objects
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Ztm<T> {
    /// No Object found, default
    None,
    /// One found
    One(T),
    /// Many found
    Many(Vec<T>)
}

impl<T> Default for Ztm<T> {
    fn default() -> Ztm<T> {
        Ztm::None
    }
}

fn is_empty<T>(i: &Ztm<T>) -> bool {
    match *i {
        Ztm::None => true,
        Ztm::Many(ref v) if v.len() == 0 => true,
        _=> false
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct LinkDetails {
    
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Link {
    Detailed(LinkDetails),
    Raw(String)
}


#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct ContextDetails {
    
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Context {
    Detailed(ContextDetails),
    Link(Link),
    Raw(String)
}

macro_rules! make_as_object {
    ($act:ident, $($attr:ident: $typ:ty,)* ) => {
        #[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
        pub struct $act {
            #[serde(skip_serializing_if="Option::is_none", default)]
            pub name: Option<String>,
            #[serde(skip_serializing_if="Option::is_none", default)]
            pub id: Option<String>,
            
            #[serde(skip_serializing_if="is_empty", default)]
            pub context: Ztm<Box<Context>>,
            #[serde(skip_serializing_if="is_empty", default)]
            pub attachment: Ztm<Box<Object>>,
            #[serde(skip_serializing_if="is_empty", default)]
            pub attributed_to: Ztm<Box<Object>>,
            
            $(
            #[serde(skip_serializing_if="is_empty", default)]
            pub $attr: $typ,
            )*
            
            #[serde(flatten)]
            pub extra: HashMap<String, String>
        }
        
        impl $act {
            pub fn new(
                id: Option<String>,
                name: Option<String>,
                $($attr: $typ,)*) -> $act {
                $act {
                    name, id,
                    $($attr,)*
                    context: Ztm::None,
                    attachment: Ztm::None,
                    attributed_to: Ztm::None,
                    extra: HashMap::default()
                }
            }
        }
    };
    ($act:ident) => {
        make_as_object!($act, );
    };
}


macro_rules! build_as_types {
    ($outer:ident, {$( $act:ident, )*}) => {
    
        $(make_as_object!($act);)*

        #[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
        #[serde(tag="type")]
        pub enum $outer {
            $( $act($act), )*
            Link(Link)
        }
    };
}

macro_rules! build_activity_types {
    ({$( $act:ident, )*}) => {
    
        $(make_as_object!($act,
            actor: Ztm<Actor>,
            object: Ztm<Object>,
            target: Ztm<Object>,
            instrument: Ztm<Object>,
            result: Ztm<Object>,
            origin: Ztm<Object>,
        );
        
        impl $act {
            pub fn by_actor(a: Actor) -> $act {
                $act::new(None, None, Ztm::One(a), Ztm::None,
                    Ztm::None, Ztm::None, Ztm::None, Ztm::None)
            }
            pub fn with_object(o: Object) -> $act {
                $act::new(None, None, Ztm::None, Ztm::One(o),
                    Ztm::None, Ztm::None, Ztm::None, Ztm::None)
            }
            pub fn by_actor_with_object(a: Actor, o: Object) -> $act {
                $act::new(None, None, Ztm::One(a), Ztm::One(o),
                    Ztm::None, Ztm::None, Ztm::None, Ztm::None)
            }
        }
        )*

        #[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
        #[serde(tag="type")]
        pub enum Activity {
            $( $act($act), )*
        }
    };
}


build_as_types!(Actor, {
    Application,
    Group,
    Organization,
    Person,
    Service,
});

build_as_types!(Object, {
    Post,
});

build_activity_types!({
    Create,
    Add,
    Remove,
});



#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(untagged)]
pub enum ActivityStreamEntity {
    Actor(Actor),
    Object(Object),
    Activity(Activity),
    Link(Link),
    Raw(String)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parsing_test() {

        let actor = Actor::Person( Person::new(Some("ben".to_owned()), None));
        let object = Object::Post( Post::new(Some("text".to_owned()), None));

        let v = vec![ActivityStreamEntity::Activity(
                Activity::Create( Create::by_actor_with_object(actor, object) )
            )];
        let out = serde_json::to_string(&v).unwrap();
        let parsed: Vec<ActivityStreamEntity> = serde_json::from_str(&out).unwrap();
        assert_eq!(v, parsed);
    }
}
