# TODO

* [ ] Add offset for conditional operation
* [x] Create wasm file
	* [x] Weealloc integration
	* [x] Pushed as npm package
* [x] Update readme.md
* [ ] Use cindex instead of csv crate
* [ ] Plotters integration
	* [ ] Multi y column plot img (left as probability, right as cost)
	* [ ] Re-use records struct if possible, but currently you can't because prob is string
	- Port prob value something into a enum variant which implments to\_string 
	* [ ] Plot object model for easier plot confiruation
* [ ] Make crate compatible with docs.rs
* [ ] Probe if library usage is ergonmic enough
* [ ] Create website
	* [x] Segregate www directory
	* [ ] Maybe github pages? -> This is somewhat important for show & prove perspective
* [ ] Use github release api for automated draft

# Implemented

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
