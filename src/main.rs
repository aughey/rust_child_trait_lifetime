trait ParentInfo<'a> {
    fn name(&self) -> &str;
}

trait Parent<'a> {
    type Info: ParentInfo<'a>;
    fn info(&'a self) -> Self::Info;
}

struct RealThing {
    name: String,
}

struct RealThingInfo<'a> {
    thing: &'a RealThing,
}

impl<'a> ParentInfo<'a> for RealThingInfo<'a> {
    fn name(&self) -> &str {
        &self.thing.name
    }
}

impl<'a> Parent<'a> for RealThing {
    type Info = RealThingInfo<'a>;

    fn info(&'a self) -> Self::Info {
        RealThingInfo { thing: self }
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;
impl Parent<'_> for UnassociatedThing {
    type Info = UnassociatedThingInfo;

    fn info(&'_ self) -> Self::Info {
        UnassociatedThingInfo {}
    }
}
impl ParentInfo<'_> for UnassociatedThingInfo {
    fn name(&self) -> &str {
        "unassociated John"
    }
}

fn generic_print<'a, T : Parent<'a>>(obj: &'a T)
{
    let info = obj.info();
    println!("{}",info.name());
}

fn generic_print_owned<'a, T : Parent<'a>>(obj: T)
{
    let info = obj.info();
    println!("{}",info.name());
}


fn main() {
    let obj = RealThing {
        name: "John".into(),
    };
    let info = obj.info();
    println!("{}", info.name());

    let obj = UnassociatedThing {};
    let info = obj.info();
    println!("{}", info.name());

    generic_print(&obj);

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
