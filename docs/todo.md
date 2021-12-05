# Implemented

* [x] Print given range of counts
* [x] Print until desired percentage
* [x] Print counts of necessary tries for desired percentage and optionally
print cost
* [x] Fixed constant logic
* [x] Bonus percentage information
* [x] Ergonomic binary usage
* [x] Qualification subcommand

# TODO

* [ ] Complete panic behaviour
* [ ] Make proper error branches
Currently every invalid option panicks with expect method
* [ ] Custom csv format mapping
- Because exsiting csv may have different headers and positions
* [ ] Ergonomic library usage
* [ ] Info redirection (File or stdout)

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
