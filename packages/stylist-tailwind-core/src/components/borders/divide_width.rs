use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "divide-x-0 " => "border-right-width: 0px; border-left-width: 0px;",
    "divide-x-2 " => "border-right-width: 0px; border-left-width: 2px;",
    "divide-x-4 " => "border-right-width: 0px; border-left-width: 4px;",
    "divide-x-8 " => "border-right-width: 0px; border-left-width: 8px;",
    "divide-x " => "border-right-width: 0px; border-left-width: 1px;",
    "divide-y-0 " => "border-top-width: 0px; border-bottom-width: 0px;",
    "divide-y-2 " => "border-top-width: 2px; border-bottom-width: 0px;",
    "divide-y-4 " => "border-top-width: 4px; border-bottom-width: 0px;",
    "divide-y-8 " => "border-top-width: 8px; border-bottom-width: 0px;",
    "divide-y " => "border-top-width: 1px; border-bottom-width: 0px;",
    "divide-y-reverse " => "--tw-divide-y-reverse: 1;",
    "divide-x-reverse " => "--tw-divide-x-reverse: 1;",
};