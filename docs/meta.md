# TODO

* [x] Make crate compatible with docs.rs
* [ ] Probe if library usage is ergonmic enough
* [ ] Create website
	* [x] Segregate www directory
	* [ ] Maybe github pages? -> This is somewhat important for show & prove perspective

* [ ] Use github release api for automated draft

# Why I ditched csv crate and used cindex instead

CSV Crate is good for concrete struct based indexing. While gcalc gets possibly
error prone + human written csv value. Thus, csv parsing should be generous and
should have fallback behaviour. Furthermore csv value can lack required
columns, which is hard to handle with csv crate.

Cindex is very lenient and can pad lacking columns with empty values. Plus can
extract value from raw csv, query into concrete form.

# Implemented

* [x] Plotters integration
	* [x] Giving alternative option is stupid. Make a plot as "builder pattern"-able state variable
	* [x] Make record as complete set of things ( Prob\_src field )
	* [x] Multi y column plot img (left as probability, right as cost)
	* [x] ~~Re-use records struct if possible, but currently you can't because prob is string~~
	- I Simply made extract prob function
	* [x] Plot object model for easier plot confiruation
	* [x] Fix index problems

* [x] Completely ditch out csv crate
* [x] Default value is Empty string, map this into agreeable default value
* [x] Proper support of column mapping through ergonomic wrapper
	- --column cost=yatti,prob=yatta
* [x] Add offset for conditional operation
* [x] Create wasm file
	* [x] Weealloc integration
	* [x] Pushed as npm package
* [x] Rounding with format is somewhat... incoherent
* [x] Config option support
	* [x] Read config
	* [x] Create config // Because I want to read option file as serialized struct

* [x] Enable qual subcommand to utilize formula when there is no reference file
* [x] Make probability as an option not an argument so that it can be used as
option file

* [x] Print given range of counts
* [x] Print until desired percentage
* [x] Print counts of necessary tries for desired percentage and optionally
print cost
* [x] Fixed constant logic
* [x] Bonus percentage information
* [x] Ergonomic binary usage
* [x] Qualification subcommand
* [x] No header option
* [x] Create a uniform csv struct for faster writing
* [x] Make csv reader get either file or csv string in a form of bufreader
* [x] Custom csv format mapping
* [x] Info redirection (File or stdout)
* [x] Make proper error branches
* [x] Make various probabilties formats supported
* [x] Fallable csv
* [x] Panic option - Only for lib usage
* [x] Make proper readme (Demo usage)
* [-] Ergonomic library usage
* [x] Prevent infinite loop

* [x] Make expected value with value option
* [-] Make several arguemtns as global for maintainability
	- You can't because, subcommand parsing only get's sub command matches not whole matches...

* [-] Ok, this crate has some missing contexts...
	- This calculator is not about at least once.
	- But about how many chances are needed if we want users to get an item.
	- Thus, geometric sequence is also a solution for only a single time
	- But, if the answer is about at least once, then it is much more easier
	- Of course, it will not work will dynamic calculation though,
	- I can also modify the case for at "least once".
	- I don't that case is the burden of this crate thus, it will not be implemented


### First usage

Expected variants are followed

- Desired percentage (Reject 100% because its not mathematically possible)
- Single try percentage

Expected output

- Count for necessary tries

### Second usage

Inputs are same with first usage

Expected output

- Show nth rows for probabilties using proper tools 
- Also create png or svg image (plotting) with optional feature

### Third usage

You give seveal inputs as form of **csv**

Expected output

- Also can be csv file
- Or proper graphical image
- Plotted image

### Fourth usage

Not only cli but also as gui tools.

### Fifth usage

But also an library that can be used in various utilities.
