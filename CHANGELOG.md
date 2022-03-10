# Changelog

## v0.3.3 - 10.03.2022

### Changes

* Chart: there are now only 3 labels on the time axis (decluttering)
* The wpm hoarder range is massively increased (allowed by the above change)
* Upgrades: 
	* rust edition to 2021
	* tui.rs to 17

## v0.3.2 - 4.12.2021

### Features

* Test starts when the first key is pressed
* Chart is now restricted by lower bound
* Chart upper bound should be more  **a e s t h e t i c**

## v0.3.1 - 1.12.2021

### Features

* -r/--recent option to show summaries of the most recent runs

### Changes

* Standard and script test results are now saved in a single table

## v0.3.0 - 10.11.2021

### Features

* Runs results are now saved in a database.
* Post screen shows comparison to record wpm achieved in a test type.
* Update `tui.rs` to `0.16.0`.

### Fixes

* Smokey will no longer panic when changing length, mods etc when script is selected.
