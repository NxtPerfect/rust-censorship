This library should be able to
- [x] Read rules from vector
- [x] Check if any of the rules match our String
- [x] Change words matching rules for chosen string, or stars
- [ ] Change strings to trait objects, so that you can use different structs
OPTIONAL:
- [ ] Configure through .toml
- [ ] Ability to set custom chars as replacement
- [ ] Add cache to store which word position and length to change, and then do it all at once
- [ ] Multithreading

Problems:
How to efficiently check vector for which one is equal to any element in another vector

- [ ] Currently it returns the rest of the string which copies it
    - What should happen is we get left part of string before word
    and right part, then we add the censored in between
