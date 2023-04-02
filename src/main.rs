trait Info<'a> {
    fn name(&self) -> &str;
}

trait HasInfo<'a> {
    type Info: Info<'a>;
    fn info(&'a self) -> Self::Info;
}

struct Thing {
    name: String,
}

struct ThingInfo<'a> {
    thing: &'a Thing,
}

impl<'a> Info<'a> for ThingInfo<'a> {
    fn name(&self) -> &str {
        &self.thing.name
    }
}

impl<'a> HasInfo<'a> for Thing {
    type Info = ThingInfo<'a>;

    fn info(&'a self) -> Self::Info {
        ThingInfo { thing: self }
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;

impl HasInfo<'_> for UnassociatedThing {
    type Info = UnassociatedThingInfo;

    fn info(&'_ self) -> Self::Info {
        UnassociatedThingInfo {}
    }
}
impl Info<'_> for UnassociatedThingInfo {
    fn name(&self) -> &str {
        "unassociated John"
    }
}

fn generic_print<'a,T>(obj: &'a T) where T : HasInfo<'a>
{
    let info = obj.info();
    println!("{}",info.name());
}

fn generic_print_owned<T>(obj: T) where T : for <'a> HasInfo<'a>
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
