use nih_plug::prelude::*;

#[derive(Params)]
struct FlatParams {
    #[id = "one"]
    pub one: BoolParam,

    #[id = "two"]
    pub two: FloatParam,

    #[id = "three"]
    pub three: IntParam,
}

impl Default for FlatParams {
    fn default() -> Self {
        FlatParams {
            one: BoolParam::new("one", true),
            two: FloatParam::new("two", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
        }
    }
}

#[derive(Params)]
struct GroupedParams {
    #[id = "one"]
    pub one: BoolParam,

    #[nested(group = "Some Group", id_prefix = "group1")]
    pub group1: FlatParams,

    #[id = "three"]
    pub three: IntParam,

    #[nested(group = "Another Group", id_prefix = "group2")]
    pub group2: FlatParams,
}

impl Default for GroupedParams {
    fn default() -> Self {
        GroupedParams {
            one: BoolParam::new("one", true),
            group1: FlatParams::default(),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
            group2: FlatParams::default(),
        }
    }
}

#[derive(Params)]
struct NestedParams {
    #[id = "one"]
    pub one: BoolParam,

    #[nested(id_prefix = "two")]
    pub two: FlatParams,

    #[id = "three"]
    pub three: IntParam,
}

impl Default for NestedParams {
    fn default() -> Self {
        NestedParams {
            one: BoolParam::new("one", true),
            two: FlatParams::default(),
            three: IntParam::new("three", 0, IntRange::Linear { min: 0, max: 100 }),
        }
    }
}

mod param_order {
    use super::*;
    #[test]
    fn flat() {
        let p = FlatParams::default();

        // Parameters must have the same order as they are defined in
        let param_ids: Vec<String> = p.param_map().into_iter().map(|(id, _, _)| id).collect();
        assert_eq!(param_ids, ["one", "two", "three",]);
    }

    #[test]
    fn grouped() {
        let p = GroupedParams::default();

        // Parameters must have the same order as they are defined in. Groups are put in the end though.
        let param_ids: Vec<String> = p.param_map().into_iter().map(|(id, _, _)| id).collect();
        assert_eq!(
            param_ids,
            [
                "one",
                "three",
                "group1_one",
                "group1_two",
                "group1_three",
                "group2_one",
                "group2_two",
                "group2_three",
            ]
        );
    }

    #[test]
    fn nested() {
        let p = NestedParams::default();

        // Parameters must have the same order as they are defined in. The position of nested parameters which are not
        // grouped explicitly is preserved.
        let param_ids: Vec<String> = p.param_map().into_iter().map(|(id, _, _)| id).collect();
        assert_eq!(
            param_ids,
            ["one", "two_one", "two_two", "two_three", "three",]
        );
    }
}