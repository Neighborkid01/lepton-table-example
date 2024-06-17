# Leptos Table Example

This example project illustrates what I believe might be a bug in the Leptos renderer.

For some reason, if I do not check the `len()` of the cells slice, it will sometimes be cached and a row will end at the prior end index instead of the actual length of the slice.
