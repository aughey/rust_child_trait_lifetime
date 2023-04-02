trait Info {
    fn name(&self) -> &str;
}

trait InfoProvider<'a> {
    type Info: Info + 'a;
    type InfoIterator : Iterator<Item = Self::Info>;

    fn info(&'a self) -> Self::Info;
    fn infos(&'a self) -> Self::InfoIterator;
}

struct Thing {
    name: String,
}

struct ThingInfo<'a> {
    thing: &'a Thing,
}

impl Info for ThingInfo<'_> {
    fn name(&self) -> &str {
        &self.thing.name
    }
}

impl<'a> InfoProvider<'a> for Thing {
    type Info = ThingInfo<'a>;
    type InfoIterator = std::vec::IntoIter<Self::Info>;

    fn info(&'a self) -> Self::Info {
        ThingInfo { thing: self }
    }
    fn infos(&'a self) -> Self::InfoIterator {
        vec![self.info()].into_iter()
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;

impl InfoProvider<'_> for UnassociatedThing {
    type Info = UnassociatedThingInfo;
    type InfoIterator = std::vec::IntoIter<Self::Info>;

    fn info(&'_ self) -> Self::Info {
        UnassociatedThingInfo {}
    }
    fn infos(&'_ self) -> Self::InfoIterator {
        vec![self.info()].into_iter()
    }
}
impl Info for UnassociatedThingInfo {
    fn name(&self) -> &str {
        "unassociated John"
    }
}

fn generic_print<'a,T>(obj: &'a T) where T : InfoProvider<'a>
{
    let info = obj.info();
    println!("{}",info.name());
}

fn generic_print_owned<T>(obj: T) where T : for <'a> InfoProvider<'a>
{
    let info = obj.info();
    println!("{}",info.name());
}


fn main() {
    let obj = Thing {
        name: "John".into(),
    };
    let info = obj.info();
    println!("{}", info.name());

    let obj = UnassociatedThing {};
    let info = obj.info();
    println!("{}", info.name());

    generic_print(&obj);
    generic_print_owned(obj);

    // This works for unassociated
    let info = {
        UnassociatedThing{}.info()
    };
    println!("{}", info.name());

    // This shouldn't work for RealThing
    // let info = {
    //     RealThing {
    //         name: "John".into()
    //     }.info()
    // };
    // println!("{}", info.name());
}
