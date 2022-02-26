use hygge::{KdlNodeBuilder, KdlValueBuilder};

fn main() {
    let v = KdlValueBuilder;
    let mut kdl = KdlNodeBuilder::new("hygge");
    let mut childs = KdlNodeBuilder::new("child");
    childs
        .bool(true)
        .prop("uuid".to_string(), v.str("123".to_string()).to_owned());
    kdl.nil()
        .nil()
        .y()
        .n()
        .child(childs.clone())
        .child(childs.clone().name("child3"))
        .str("hello")
        .child(KdlNodeBuilder::new("child2"));
    println!("{}", kdl.build());

    let mut kdl = KdlNodeBuilder::new("hygge");
    kdl.nil()
        .nil()
        .y()
        .n()
        .child(childs.clone())
        .child(childs.clone().name("child3"))
        .str("hello")
        .child(KdlNodeBuilder::new("child2"))
        .child(KdlNodeBuilder::new("child4"))
        .prop("asd".to_string(), v.flt(1.into()));
    let two = kdl.clone();
    kdl.child(two.name("hygge2"))
        .prop("asd".to_string(), v.str(2.to_string()));
    println!("{}", kdl.build());
}

/*
    ideas
        - unify str vs String
        - unify &mut self vs Self
*/
