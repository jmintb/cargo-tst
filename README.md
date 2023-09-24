# cargo-tst: fuzzy test search

cargo-tst is a small cargo extension to allow fuzzy finding of tests. It is currently implemented in a quick MVP fashion. Meaning that the code is ugly, many edge(and non edge)cases
 are not handled. It is however already quite usefull if you have a project with many files and you are tired having to type in the exact name of the test you want to run.

Please leave a star if this is something you are interested in. If others find this useful I will polish it and publish to crates.io. 

### Usage

#### Installation

I am holding off on publishing this to crates.io until I can gauge if others find this useful.

`cargo install --git https://github.com/jmintb/cargo-tst.git`

### Usage

cd into the project and type `cargo tst search_term` where `search_term` is the term used to match against test names.
The five closest matching names will be presented and you can select which one to run. After selecting a test `cargo tst` can
ben run without a search term to rerun the same test. The last run test is project specific so there is no overlap when switching projects.


