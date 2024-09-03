# Roadmap

## Version 1.0.0

- [x] Core Functions
  - [x] Loading images from directory
  - [x] Merging input images in memory
  - [x] Split finder
  - [x] Splitter/exporter
- [ ] GUI
  - [ ] Configuration Page
    - [ ] Input/Output filepath configuration
    - [ ] Output configuration
      - [ ] Filetype selector (`.jpg`/`.jpeg`/`.png`/`.webp`) with optional quality parameter depending on extension
      - [ ] Optional fixed width (otherwise image with smallest width will be selected)
      - [ ] Max page height
        - [ ] Hard limit page height depending on chosen output file extension
  - [ ] Selector for split method
    - [ ] Hard splitting (Split at max page height every time)
      - Same as smart splitting with sensitivity set to 0%.
    - [ ] Smart splitting (Use adjacent pixel comparisons + multiline checks to determine where to split)
      - [ ] Scan line step size (in pixels)
        - [ ] Have checkbox to automatically calculate the based on a ratio & set page width
  - [ ] Double-check/Restitch Page
    - [ ] Show split locations
    - [ ] Modify split locations by scrolling
    - [ ] Add splits anywhere
    - [ ] Last page should be shown with half of viewport height buffer added to the bottom (to allow for splitting near the very end)
    - [ ] Show page heights for current configuration
    - [ ] Warn when page height exceeds set max page height
- [ ] Nice-to-haves
  - [ ] Batch mode
  - [ ] User guide
  - [ ] Configuration profiles
  - [x] Debug mode (to analyze the scan lines and their max pixel diff values)
