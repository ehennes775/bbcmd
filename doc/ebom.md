# Engineering Bill of Materials

The EBOM feature of `bbcmd` allows one or more symbols, in the schematics, to refer to a purchased part.

An example line item in the EBOM follows:

```
   0|100001.json     |12 V            |Diode, TVS, Unidirectional, 12 V, SMA
        D3,D2
        Bourns Inc          |SMAJ12A
        Diodes Incorporated |SMAJ12A-13-F
        Littelfuse Inc      |SMAJ12A
```

This line items shows D2 and D3 as unidirectional TVS diodes. But, any of the three manufacturers and manufactrerer part numbers listed, can be used for D2 and D3.
