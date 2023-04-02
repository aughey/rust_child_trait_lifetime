trait Info {
    fn name(&self) -> &str;
}

trait InfoProvider {
    type Item<'a>: Info + 'a
    where
        Self: 'a;
    type InfoIterator<'a>: Iterator<Item = Self::Item<'a>> where Self: 'a;
    fn info<'a>(&'a self) -> Self::Item<'a>;
    fn infos<'a>(&'a self) -> Self::InfoIterator<'a>;
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

impl InfoProvider for Thing {
    type Item<'a> = ThingInfo<'a>;
    type InfoIterator<'a> = std::vec::IntoIter<Self::Item<'a>>;

    fn info<'a>(&'a self) -> Self::Item<'a> {
        ThingInfo { thing: self }
    }
    fn infos<'a>(&'a self) -> Self::InfoIterator<'a> {
        vec![self.info()].into_iter()
    }
}

struct UnassociatedThing;
struct UnassociatedThingInfo;

impl InfoProvider for UnassociatedThing {
    type Item<'a> = UnassociatedThingInfo;
    type InfoIterator<'a> = std::vec::IntoIter<Self::Item<'a>>;

    fn info(&self) -> Self::Item<'_> {
        UnassociatedThingInfo {}
    }
    fn infos(&self) -> Self::InfoIterator<'_> {
        vec![self.info()].into_iter()
    }
}
impl Info for UnassociatedThingInfo {
    fn name(&self) -> &str {
        "unassociated John"
    }
}

fn generic_print<'a, T>(obj: &'a T)
where
    T: InfoProvider,
{
    let info = obj.info();
    println!("{}", info.name());
}

fn generic_print_owned<T>(obj: T)
where
    T: for<'a> InfoProvider,
{
    let info = obj.info();
    println!("{}", info.name());
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
    let info = { UnassociatedThing {}.info() };
    println!("{}", info.name());

    // This shouldn't work for RealThing
    // let info = {
    //     RealThing {
    //         name: "John".into()
    //     }.info()
    // };
    // println!("{}", info.name());
}
