use hygge::{KdlNodeBuilder, KdlValueBuilder};

fn main() {
    // 0.3.0
    //     let v = KdlValueBuilder;
    //     let mut kdl = KdlNodeBuilder::new("hygge");
    //     let mut childs = KdlNodeBuilder::new("child")
    //         .bool(true)
    //         .prop("uuid".to_string(), v.str("123".to_string()).to_owned());
    //     kdl = kdl
    //         .nil()
    //         .nil()
    //         .y()
    //         .n()
    //         .child(childs.clone())
    //         .child(childs.clone().name("child3"))
    //         .str("hello")
    //         .child(KdlNodeBuilder::new("child2"));
    //     println!("{}", kdl.build());

    //     let mut kdl = KdlNodeBuilder::new("hygge")
    //         .nil()
    //         .nil()
    //         .y()
    //         .n()
    //         .child(childs.clone())
    //         .child(childs.clone().name("child3"))
    //         .str("hello")
    //         .child(KdlNodeBuilder::new("child2"))
    //         .child(KdlNodeBuilder::new("child4"))
    //         .prop("asd".to_string(), v.flt(1.into()));
    //     let two = &kdl.clone().child(kdl.name("hygge2")).prop("asd", v.str(2));
    //     println!("{}", two.build());

    // 0.4.0
    let v = KdlValueBuilder;
    println!(
        "{}",
        KdlNodeBuilder::new(&mut "hygge".to_string())
            .nil()
            .nil()
            .y()
            .n()
            .str(&mut "hello".to_string())
            .prop(&mut "asd".to_string(), v.flt(1.into()))
            .build()
    );
}

/*
    ideas
        - unify str vs String
        - unify &mut self vs Self
*/
