# TODO

* [ ] Enable qual subcommand to utilize formula when there is no reference file
* [ ] Make probabilty as an option not an argument so that it can be used as
option file
* [ ] Config option support
	* [ ] Read config
	* [ ] Create config // Because I want to read option file as serialized struct
- Because gcalc often comes with too verbose options
- This comes first and user can override option file with external flags
for e.g.
```
gcalc --option option.json
===
% option.json
{
	"probabilty" : 0.3,
	"precision" : 2,
	"format" : "gfm",
	"target" : 0.10,
	"fallable" : true,
}
```
* [ ] Create wasm file
* [ ] Use github release api for automated draft

# Implemented

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
