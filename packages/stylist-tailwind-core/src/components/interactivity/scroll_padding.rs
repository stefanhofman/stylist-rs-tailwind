use phf::phf_map;

pub static CLASSES: phf::Map<&'static str, &'static str> = phf_map! {
    "scroll-p-0" => "scroll-padding: 0px;",
    "scroll-px-0" => "scroll-padding-left: 0px; scroll-padding-right: 0px;",
    "scroll-py-0" => "scroll-padding-top: 0px; scroll-padding-bottom: 0px;",
    "scroll-ps-0" => "scroll-padding-inline-start: 0px;",
    "scroll-pe-0" => "scroll-padding-inline-end: 0px;",
    "scroll-pt-0" => "scroll-padding-top: 0px;",
    "scroll-pr-0" => "scroll-padding-right: 0px;",
    "scroll-pb-0" => "scroll-padding-bottom: 0px;",
    "scroll-pl-0" => "scroll-padding-left: 0px;",
    "scroll-p-px" => "scroll-padding: 1px;",
    "scroll-px-px" => "scroll-padding-left: 1px; scroll-padding-right: 1px;",
    "scroll-py-px" => "scroll-padding-top: 1px; scroll-padding-bottom: 1px;",
    "scroll-ps-px" => "scroll-padding-inline-start: 1px;",
    "scroll-pe-px" => "scroll-padding-inline-end: 1px;",
    "scroll-pt-px" => "scroll-padding-top: 1px;",
    "scroll-pr-px" => "scroll-padding-right: 1px;",
    "scroll-pb-px" => "scroll-padding-bottom: 1px;",
    "scroll-pl-px" => "scroll-padding-left: 1px;",
    "scroll-p-0.5" => "scroll-padding: 0.125rem;",
    "scroll-px-0.5" => "scroll-padding-left: 0.125rem; scroll-padding-right: 0.125rem;",
    "scroll-py-0.5" => "scroll-padding-top: 0.125rem; scroll-padding-bottom: 0.125rem;",
    "scroll-ps-0.5" => "scroll-padding-inline-start: 0.125rem;",
    "scroll-pe-0.5" => "scroll-padding-inline-end: 0.125rem;",
    "scroll-pt-0.5" => "scroll-padding-top: 0.125rem;",
    "scroll-pr-0.5" => "scroll-padding-right: 0.125rem;",
    "scroll-pb-0.5" => "scroll-padding-bottom: 0.125rem;",
    "scroll-pl-0.5" => "scroll-padding-left: 0.125rem;",
    "scroll-p-1" => "scroll-padding: 0.25rem;",
    "scroll-px-1" => "scroll-padding-left: 0.25rem; scroll-padding-right: 0.25rem;",
    "scroll-py-1" => "scroll-padding-top: 0.25rem; scroll-padding-bottom: 0.25rem;",
    "scroll-ps-1" => "scroll-padding-inline-start: 0.25rem;",
    "scroll-pe-1" => "scroll-padding-inline-end: 0.25rem;",
    "scroll-pt-1" => "scroll-padding-top: 0.25rem;",
    "scroll-pr-1" => "scroll-padding-right: 0.25rem;",
    "scroll-pb-1" => "scroll-padding-bottom: 0.25rem;",
    "scroll-pl-1" => "scroll-padding-left: 0.25rem;",
    "scroll-p-1.5" => "scroll-padding: 0.375rem;",
    "scroll-px-1.5" => "scroll-padding-left: 0.375rem; scroll-padding-right: 0.375rem;",
    "scroll-py-1.5" => "scroll-padding-top: 0.375rem; scroll-padding-bottom: 0.375rem;",
    "scroll-ps-1.5" => "scroll-padding-inline-start: 0.375rem;",
    "scroll-pe-1.5" => "scroll-padding-inline-end: 0.375rem;",
    "scroll-pt-1.5" => "scroll-padding-top: 0.375rem;",
    "scroll-pr-1.5" => "scroll-padding-right: 0.375rem;",
    "scroll-pb-1.5" => "scroll-padding-bottom: 0.375rem;",
    "scroll-pl-1.5" => "scroll-padding-left: 0.375rem;",
    "scroll-p-2" => "scroll-padding: 0.5rem;",
    "scroll-px-2" => "scroll-padding-left: 0.5rem; scroll-padding-right: 0.5rem;",
    "scroll-py-2" => "scroll-padding-top: 0.5rem; scroll-padding-bottom: 0.5rem;",
    "scroll-ps-2" => "scroll-padding-inline-start: 0.5rem;",
    "scroll-pe-2" => "scroll-padding-inline-end: 0.5rem;",
    "scroll-pt-2" => "scroll-padding-top: 0.5rem;",
    "scroll-pr-2" => "scroll-padding-right: 0.5rem;",
    "scroll-pb-2" => "scroll-padding-bottom: 0.5rem;",
    "scroll-pl-2" => "scroll-padding-left: 0.5rem;",
    "scroll-p-2.5" => "scroll-padding: 0.625rem;",
    "scroll-px-2.5" => "scroll-padding-left: 0.625rem; scroll-padding-right: 0.625rem;",
    "scroll-py-2.5" => "scroll-padding-top: 0.625rem; scroll-padding-bottom: 0.625rem;",
    "scroll-ps-2.5" => "scroll-padding-inline-start: 0.625rem;",
    "scroll-pe-2.5" => "scroll-padding-inline-end: 0.625rem;",
    "scroll-pt-2.5" => "scroll-padding-top: 0.625rem;",
    "scroll-pr-2.5" => "scroll-padding-right: 0.625rem;",
    "scroll-pb-2.5" => "scroll-padding-bottom: 0.625rem;",
    "scroll-pl-2.5" => "scroll-padding-left: 0.625rem;",
    "scroll-p-3" => "scroll-padding: 0.75rem;",
    "scroll-px-3" => "scroll-padding-left: 0.75rem; scroll-padding-right: 0.75rem;",
    "scroll-py-3" => "scroll-padding-top: 0.75rem; scroll-padding-bottom: 0.75rem;",
    "scroll-ps-3" => "scroll-padding-inline-start: 0.75rem;",
    "scroll-pe-3" => "scroll-padding-inline-end: 0.75rem;",
    "scroll-pt-3" => "scroll-padding-top: 0.75rem;",
    "scroll-pr-3" => "scroll-padding-right: 0.75rem;",
    "scroll-pb-3" => "scroll-padding-bottom: 0.75rem;",
    "scroll-pl-3" => "scroll-padding-left: 0.75rem;",
    "scroll-p-3.5" => "scroll-padding: 0.875rem;",
    "scroll-px-3.5" => "scroll-padding-left: 0.875rem; scroll-padding-right: 0.875rem;",
    "scroll-py-3.5" => "scroll-padding-top: 0.875rem; scroll-padding-bottom: 0.875rem;",
    "scroll-ps-3.5" => "scroll-padding-inline-start: 0.875rem;",
    "scroll-pe-3.5" => "scroll-padding-inline-end: 0.875rem;",
    "scroll-pt-3.5" => "scroll-padding-top: 0.875rem;",
    "scroll-pr-3.5" => "scroll-padding-right: 0.875rem;",
    "scroll-pb-3.5" => "scroll-padding-bottom: 0.875rem;",
    "scroll-pl-3.5" => "scroll-padding-left: 0.875rem;",
    "scroll-p-4" => "scroll-padding: 1rem;",
    "scroll-px-4" => "scroll-padding-left: 1rem; scroll-padding-right: 1rem;",
    "scroll-py-4" => "scroll-padding-top: 1rem; scroll-padding-bottom: 1rem;",
    "scroll-ps-4" => "scroll-padding-inline-start: 1rem;",
    "scroll-pe-4" => "scroll-padding-inline-end: 1rem;",
    "scroll-pt-4" => "scroll-padding-top: 1rem;",
    "scroll-pr-4" => "scroll-padding-right: 1rem;",
    "scroll-pb-4" => "scroll-padding-bottom: 1rem;",
    "scroll-pl-4" => "scroll-padding-left: 1rem;",
    "scroll-p-5" => "scroll-padding: 1.25rem;",
    "scroll-px-5" => "scroll-padding-left: 1.25rem; scroll-padding-right: 1.25rem;",
    "scroll-py-5" => "scroll-padding-top: 1.25rem; scroll-padding-bottom: 1.25rem;",
    "scroll-ps-5" => "scroll-padding-inline-start: 1.25rem;",
    "scroll-pe-5" => "scroll-padding-inline-end: 1.25rem;",
    "scroll-pt-5" => "scroll-padding-top: 1.25rem;",
    "scroll-pr-5" => "scroll-padding-right: 1.25rem;",
    "scroll-pb-5" => "scroll-padding-bottom: 1.25rem;",
    "scroll-pl-5" => "scroll-padding-left: 1.25rem;",
    "scroll-p-6" => "scroll-padding: 1.5rem;",
    "scroll-px-6" => "scroll-padding-left: 1.5rem; scroll-padding-right: 1.5rem;",
    "scroll-py-6" => "scroll-padding-top: 1.5rem; scroll-padding-bottom: 1.5rem;",
    "scroll-ps-6" => "scroll-padding-inline-start: 1.5rem;",
    "scroll-pe-6" => "scroll-padding-inline-end: 1.5rem;",
    "scroll-pt-6" => "scroll-padding-top: 1.5rem;",
    "scroll-pr-6" => "scroll-padding-right: 1.5rem;",
    "scroll-pb-6" => "scroll-padding-bottom: 1.5rem;",
    "scroll-pl-6" => "scroll-padding-left: 1.5rem;",
    "scroll-p-7" => "scroll-padding: 1.75rem;",
    "scroll-px-7" => "scroll-padding-left: 1.75rem; scroll-padding-right: 1.75rem;",
    "scroll-py-7" => "scroll-padding-top: 1.75rem; scroll-padding-bottom: 1.75rem;",
    "scroll-ps-7" => "scroll-padding-inline-start: 1.75rem;",
    "scroll-pe-7" => "scroll-padding-inline-end: 1.75rem;",
    "scroll-pt-7" => "scroll-padding-top: 1.75rem;",
    "scroll-pr-7" => "scroll-padding-right: 1.75rem;",
    "scroll-pb-7" => "scroll-padding-bottom: 1.75rem;",
    "scroll-pl-7" => "scroll-padding-left: 1.75rem;",
    "scroll-p-8" => "scroll-padding: 2rem;",
    "scroll-px-8" => "scroll-padding-left: 2rem; scroll-padding-right: 2rem;",
    "scroll-py-8" => "scroll-padding-top: 2rem; scroll-padding-bottom: 2rem;",
    "scroll-ps-8" => "scroll-padding-inline-start: 2rem;",
    "scroll-pe-8" => "scroll-padding-inline-end: 2rem;",
    "scroll-pt-8" => "scroll-padding-top: 2rem;",
    "scroll-pr-8" => "scroll-padding-right: 2rem;",
    "scroll-pb-8" => "scroll-padding-bottom: 2rem;",
    "scroll-pl-8" => "scroll-padding-left: 2rem;",
    "scroll-p-9" => "scroll-padding: 2.25rem;",
    "scroll-px-9" => "scroll-padding-left: 2.25rem; scroll-padding-right: 2.25rem;",
    "scroll-py-9" => "scroll-padding-top: 2.25rem; scroll-padding-bottom: 2.25rem;",
    "scroll-ps-9" => "scroll-padding-inline-start: 2.25rem;",
    "scroll-pe-9" => "scroll-padding-inline-end: 2.25rem;",
    "scroll-pt-9" => "scroll-padding-top: 2.25rem;",
    "scroll-pr-9" => "scroll-padding-right: 2.25rem;",
    "scroll-pb-9" => "scroll-padding-bottom: 2.25rem;",
    "scroll-pl-9" => "scroll-padding-left: 2.25rem;",
    "scroll-p-10" => "scroll-padding: 2.5rem;",
    "scroll-px-10" => "scroll-padding-left: 2.5rem; scroll-padding-right: 2.5rem;",
    "scroll-py-10" => "scroll-padding-top: 2.5rem; scroll-padding-bottom: 2.5rem;",
    "scroll-ps-10" => "scroll-padding-inline-start: 2.5rem;",
    "scroll-pe-10" => "scroll-padding-inline-end: 2.5rem;",
    "scroll-pt-10" => "scroll-padding-top: 2.5rem;",
    "scroll-pr-10" => "scroll-padding-right: 2.5rem;",
    "scroll-pb-10" => "scroll-padding-bottom: 2.5rem;",
    "scroll-pl-10" => "scroll-padding-left: 2.5rem;",
    "scroll-p-11" => "scroll-padding: 2.75rem;",
    "scroll-px-11" => "scroll-padding-left: 2.75rem; scroll-padding-right: 2.75rem;",
    "scroll-py-11" => "scroll-padding-top: 2.75rem; scroll-padding-bottom: 2.75rem;",
    "scroll-ps-11" => "scroll-padding-inline-start: 2.75rem;",
    "scroll-pe-11" => "scroll-padding-inline-end: 2.75rem;",
    "scroll-pt-11" => "scroll-padding-top: 2.75rem;",
    "scroll-pr-11" => "scroll-padding-right: 2.75rem;",
    "scroll-pb-11" => "scroll-padding-bottom: 2.75rem;",
    "scroll-pl-11" => "scroll-padding-left: 2.75rem;",
    "scroll-p-12" => "scroll-padding: 3rem;",
    "scroll-px-12" => "scroll-padding-left: 3rem; scroll-padding-right: 3rem;",
    "scroll-py-12" => "scroll-padding-top: 3rem; scroll-padding-bottom: 3rem;",
    "scroll-ps-12" => "scroll-padding-inline-start: 3rem;",
    "scroll-pe-12" => "scroll-padding-inline-end: 3rem;",
    "scroll-pt-12" => "scroll-padding-top: 3rem;",
    "scroll-pr-12" => "scroll-padding-right: 3rem;",
    "scroll-pb-12" => "scroll-padding-bottom: 3rem;",
    "scroll-pl-12" => "scroll-padding-left: 3rem;",
    "scroll-p-14" => "scroll-padding: 3.5rem;",
    "scroll-px-14" => "scroll-padding-left: 3.5rem; scroll-padding-right: 3.5rem;",
    "scroll-py-14" => "scroll-padding-top: 3.5rem; scroll-padding-bottom: 3.5rem;",
    "scroll-ps-14" => "scroll-padding-inline-start: 3.5rem;",
    "scroll-pe-14" => "scroll-padding-inline-end: 3.5rem;",
    "scroll-pt-14" => "scroll-padding-top: 3.5rem;",
    "scroll-pr-14" => "scroll-padding-right: 3.5rem;",
    "scroll-pb-14" => "scroll-padding-bottom: 3.5rem;",
    "scroll-pl-14" => "scroll-padding-left: 3.5rem;",
    "scroll-p-16" => "scroll-padding: 4rem;",
    "scroll-px-16" => "scroll-padding-left: 4rem; scroll-padding-right: 4rem;",
    "scroll-py-16" => "scroll-padding-top: 4rem; scroll-padding-bottom: 4rem;",
    "scroll-ps-16" => "scroll-padding-inline-start: 4rem;",
    "scroll-pe-16" => "scroll-padding-inline-end: 4rem;",
    "scroll-pt-16" => "scroll-padding-top: 4rem;",
    "scroll-pr-16" => "scroll-padding-right: 4rem;",
    "scroll-pb-16" => "scroll-padding-bottom: 4rem;",
    "scroll-pl-16" => "scroll-padding-left: 4rem;",
    "scroll-p-20" => "scroll-padding: 5rem;",
    "scroll-px-20" => "scroll-padding-left: 5rem; scroll-padding-right: 5rem;",
    "scroll-py-20" => "scroll-padding-top: 5rem; scroll-padding-bottom: 5rem;",
    "scroll-ps-20" => "scroll-padding-inline-start: 5rem;",
    "scroll-pe-20" => "scroll-padding-inline-end: 5rem;",
    "scroll-pt-20" => "scroll-padding-top: 5rem;",
    "scroll-pr-20" => "scroll-padding-right: 5rem;",
    "scroll-pb-20" => "scroll-padding-bottom: 5rem;",
    "scroll-pl-20" => "scroll-padding-left: 5rem;",
    "scroll-p-24" => "scroll-padding: 6rem;",
    "scroll-px-24" => "scroll-padding-left: 6rem; scroll-padding-right: 6rem;",
    "scroll-py-24" => "scroll-padding-top: 6rem; scroll-padding-bottom: 6rem;",
    "scroll-ps-24" => "scroll-padding-inline-start: 6rem;",
    "scroll-pe-24" => "scroll-padding-inline-end: 6rem;",
    "scroll-pt-24" => "scroll-padding-top: 6rem;",
    "scroll-pr-24" => "scroll-padding-right: 6rem;",
    "scroll-pb-24" => "scroll-padding-bottom: 6rem;",
    "scroll-pl-24" => "scroll-padding-left: 6rem;",
    "scroll-p-28" => "scroll-padding: 7rem;",
    "scroll-px-28" => "scroll-padding-left: 7rem; scroll-padding-right: 7rem;",
    "scroll-py-28" => "scroll-padding-top: 7rem; scroll-padding-bottom: 7rem;",
    "scroll-ps-28" => "scroll-padding-inline-start: 7rem;",
    "scroll-pe-28" => "scroll-padding-inline-end: 7rem;",
    "scroll-pt-28" => "scroll-padding-top: 7rem;",
    "scroll-pr-28" => "scroll-padding-right: 7rem;",
    "scroll-pb-28" => "scroll-padding-bottom: 7rem;",
    "scroll-pl-28" => "scroll-padding-left: 7rem;",
    "scroll-p-32" => "scroll-padding: 8rem;",
    "scroll-px-32" => "scroll-padding-left: 8rem; scroll-padding-right: 8rem;",
    "scroll-py-32" => "scroll-padding-top: 8rem; scroll-padding-bottom: 8rem;",
    "scroll-ps-32" => "scroll-padding-inline-start: 8rem;",
    "scroll-pe-32" => "scroll-padding-inline-end: 8rem;",
    "scroll-pt-32" => "scroll-padding-top: 8rem;",
    "scroll-pr-32" => "scroll-padding-right: 8rem;",
    "scroll-pb-32" => "scroll-padding-bottom: 8rem;",
    "scroll-pl-32" => "scroll-padding-left: 8rem;",
    "scroll-p-36" => "scroll-padding: 9rem;",
    "scroll-px-36" => "scroll-padding-left: 9rem; scroll-padding-right: 9rem;",
    "scroll-py-36" => "scroll-padding-top: 9rem; scroll-padding-bottom: 9rem;",
    "scroll-ps-36" => "scroll-padding-inline-start: 9rem;",
    "scroll-pe-36" => "scroll-padding-inline-end: 9rem;",
    "scroll-pt-36" => "scroll-padding-top: 9rem;",
    "scroll-pr-36" => "scroll-padding-right: 9rem;",
    "scroll-pb-36" => "scroll-padding-bottom: 9rem;",
    "scroll-pl-36" => "scroll-padding-left: 9rem;",
    "scroll-p-40" => "scroll-padding: 10rem;",
    "scroll-px-40" => "scroll-padding-left: 10rem; scroll-padding-right: 10rem;",
    "scroll-py-40" => "scroll-padding-top: 10rem; scroll-padding-bottom: 10rem;",
    "scroll-ps-40" => "scroll-padding-inline-start: 10rem;",
    "scroll-pe-40" => "scroll-padding-inline-end: 10rem;",
    "scroll-pt-40" => "scroll-padding-top: 10rem;",
    "scroll-pr-40" => "scroll-padding-right: 10rem;",
    "scroll-pb-40" => "scroll-padding-bottom: 10rem;",
    "scroll-pl-40" => "scroll-padding-left: 10rem;",
    "scroll-p-44" => "scroll-padding: 11rem;",
    "scroll-px-44" => "scroll-padding-left: 11rem; scroll-padding-right: 11rem;",
    "scroll-py-44" => "scroll-padding-top: 11rem; scroll-padding-bottom: 11rem;",
    "scroll-ps-44" => "scroll-padding-inline-start: 11rem;",
    "scroll-pe-44" => "scroll-padding-inline-end: 11rem;",
    "scroll-pt-44" => "scroll-padding-top: 11rem;",
    "scroll-pr-44" => "scroll-padding-right: 11rem;",
    "scroll-pb-44" => "scroll-padding-bottom: 11rem;",
    "scroll-pl-44" => "scroll-padding-left: 11rem;",
    "scroll-p-48" => "scroll-padding: 12rem;",
    "scroll-px-48" => "scroll-padding-left: 12rem; scroll-padding-right: 12rem;",
    "scroll-py-48" => "scroll-padding-top: 12rem; scroll-padding-bottom: 12rem;",
    "scroll-ps-48" => "scroll-padding-inline-start: 12rem;",
    "scroll-pe-48" => "scroll-padding-inline-end: 12rem;",
    "scroll-pt-48" => "scroll-padding-top: 12rem;",
    "scroll-pr-48" => "scroll-padding-right: 12rem;",
    "scroll-pb-48" => "scroll-padding-bottom: 12rem;",
    "scroll-pl-48" => "scroll-padding-left: 12rem;",
    "scroll-p-52" => "scroll-padding: 13rem;",
    "scroll-px-52" => "scroll-padding-left: 13rem; scroll-padding-right: 13rem;",
    "scroll-py-52" => "scroll-padding-top: 13rem; scroll-padding-bottom: 13rem;",
    "scroll-ps-52" => "scroll-padding-inline-start: 13rem;",
    "scroll-pe-52" => "scroll-padding-inline-end: 13rem;",
    "scroll-pt-52" => "scroll-padding-top: 13rem;",
    "scroll-pr-52" => "scroll-padding-right: 13rem;",
    "scroll-pb-52" => "scroll-padding-bottom: 13rem;",
    "scroll-pl-52" => "scroll-padding-left: 13rem;",
    "scroll-p-56" => "scroll-padding: 14rem;",
    "scroll-px-56" => "scroll-padding-left: 14rem; scroll-padding-right: 14rem;",
    "scroll-py-56" => "scroll-padding-top: 14rem; scroll-padding-bottom: 14rem;",
    "scroll-ps-56" => "scroll-padding-inline-start: 14rem;",
    "scroll-pe-56" => "scroll-padding-inline-end: 14rem;",
    "scroll-pt-56" => "scroll-padding-top: 14rem;",
    "scroll-pr-56" => "scroll-padding-right: 14rem;",
    "scroll-pb-56" => "scroll-padding-bottom: 14rem;",
    "scroll-pl-56" => "scroll-padding-left: 14rem;",
    "scroll-p-60" => "scroll-padding: 15rem;",
    "scroll-px-60" => "scroll-padding-left: 15rem; scroll-padding-right: 15rem;",
    "scroll-py-60" => "scroll-padding-top: 15rem; scroll-padding-bottom: 15rem;",
    "scroll-ps-60" => "scroll-padding-inline-start: 15rem;",
    "scroll-pe-60" => "scroll-padding-inline-end: 15rem;",
    "scroll-pt-60" => "scroll-padding-top: 15rem;",
    "scroll-pr-60" => "scroll-padding-right: 15rem;",
    "scroll-pb-60" => "scroll-padding-bottom: 15rem;",
    "scroll-pl-60" => "scroll-padding-left: 15rem;",
    "scroll-p-64" => "scroll-padding: 16rem;",
    "scroll-px-64" => "scroll-padding-left: 16rem; scroll-padding-right: 16rem;",
    "scroll-py-64" => "scroll-padding-top: 16rem; scroll-padding-bottom: 16rem;",
    "scroll-ps-64" => "scroll-padding-inline-start: 16rem;",
    "scroll-pe-64" => "scroll-padding-inline-end: 16rem;",
    "scroll-pt-64" => "scroll-padding-top: 16rem;",
    "scroll-pr-64" => "scroll-padding-right: 16rem;",
    "scroll-pb-64" => "scroll-padding-bottom: 16rem;",
    "scroll-pl-64" => "scroll-padding-left: 16rem;",
    "scroll-p-72" => "scroll-padding: 18rem;",
    "scroll-px-72" => "scroll-padding-left: 18rem; scroll-padding-right: 18rem;",
    "scroll-py-72" => "scroll-padding-top: 18rem; scroll-padding-bottom: 18rem;",
    "scroll-ps-72" => "scroll-padding-inline-start: 18rem;",
    "scroll-pe-72" => "scroll-padding-inline-end: 18rem;",
    "scroll-pt-72" => "scroll-padding-top: 18rem;",
    "scroll-pr-72" => "scroll-padding-right: 18rem;",
    "scroll-pb-72" => "scroll-padding-bottom: 18rem;",
    "scroll-pl-72" => "scroll-padding-left: 18rem;",
    "scroll-p-80" => "scroll-padding: 20rem;",
    "scroll-px-80" => "scroll-padding-left: 20rem; scroll-padding-right: 20rem;",
    "scroll-py-80" => "scroll-padding-top: 20rem; scroll-padding-bottom: 20rem;",
    "scroll-ps-80" => "scroll-padding-inline-start: 20rem;",
    "scroll-pe-80" => "scroll-padding-inline-end: 20rem;",
    "scroll-pt-80" => "scroll-padding-top: 20rem;",
    "scroll-pr-80" => "scroll-padding-right: 20rem;",
    "scroll-pb-80" => "scroll-padding-bottom: 20rem;",
    "scroll-pl-80" => "scroll-padding-left: 20rem;",
    "scroll-p-96" => "scroll-padding: 24rem;",
    "scroll-px-96" => "scroll-padding-left: 24rem; scroll-padding-right: 24rem;",
    "scroll-py-96" => "scroll-padding-top: 24rem; scroll-padding-bottom: 24rem;",
    "scroll-ps-96" => "scroll-padding-inline-start: 24rem;",
    "scroll-pe-96" => "scroll-padding-inline-end: 24rem;",
    "scroll-pt-96" => "scroll-padding-top: 24rem;",
    "scroll-pr-96" => "scroll-padding-right: 24rem;",
    "scroll-pb-96" => "scroll-padding-bottom: 24rem;",
    "scroll-pl-96" => "scroll-padding-left: 24rem;",
};