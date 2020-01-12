# SCDs
These Specification Control Drawings (SCDs) provide the following:

* Allow a symbol in a schematic to represent common components supplied by multiple manufacturers
* Allow manufacturing to select alternate parts based on cost and availability
* Reduce mistakes populating attributes in the schematic

For specifying symbols that represent a purchased part, only the following attributes need to be set in the symbol to generate a Engineering BOM (EBOM):

| Name | Description |
| - | - |
| scd | The filename of the SCD file including extension |
| value | The value of the part |

## Description
The field contains a description of the part that will appear in the EBOM. The `$(value)` will expand to the value specified in the schematic.

```json
    "description": "Diode, TVS, Unidirectional, $(value), SMA",
```

## Footprints
The footprints field contains an array of the footprint filenames.

```json
    "footprints": [
        "ech-diode-sma.fp"
    ],
```
Tools will check the footprint in the file to ensure is matches one of the allowable footprints. (With a single footprint available, tools could populate the symbol attribute with the correct value.)

## Groups
This field contains an array of groups of parts. 

* All parts within the same SCD file meet the same set of specifications, except for the value.
* All parts withing a group share the same value, and meet the overall set of specifications.

An example object within the groups array follows:

```json
    {
        "value": "4.75 kΩ",
        "parts": [
            {
                "m": "Bourns Inc",
                "p": "CR0603-FX-4751ELF"
            }
        ]
    },
```

### Value
This field specifies the value that all components within the group share. THE EBOM utility will oacte groups by matching the value attribute, from the symbol in the schematic, with this field.

### Parts
The parts field contains an array of objects with the following fields:

| Name | Description |
| - | - |
| m | The name of the manufacturer |
| p | The manufacturer part number |
