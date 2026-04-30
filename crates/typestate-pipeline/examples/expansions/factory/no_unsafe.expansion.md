No public-API diff. The **Minimal** sketch above applies verbatim;
`finalize`'s impl uses `<Flag as Storage<T>>::finalize_or` internally
instead of an `IS_SET` branch, but that's invisible at the call site.
