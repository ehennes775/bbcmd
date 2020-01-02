# Reference Designators

1. __Initially, reset all reference designators.__ When initial designs are cut and paste from other designs, the reference designators may need to be reset. Optionally, reset the reference designaors applicable the the other projects with the following command.

    ```
    bbcmd refdes -r design-1.sch design-2.sch design-3.sch
    ```

2. __Create an initial counters file.__ The counters file ensures that reference designators are not reused after removing components from the design. The counters file maintains the reference designator with the greatest number for each prefix. Archive the counters file with the design.

    ```
    bbcmd refdes -c refdes.cnt design-1.sch design-2.sch design-3.sch
    ```

3. __Assign heterogenous and homogenous components manually.__ When multiple symbols represent a signle component, no mechanism exists to represent the associations. For these cases, the reference designators must be assigned manually.

4. __Assign remaining components automatically.__ All unassigned reference designators will be assigned with the following command:

    ```
    bbcmd refdes -a -c refdes.cnt design-1.sch design-2.sch design-3.sch
    ```

5. __Update reference designators each revision.__ Repeat steps 3 and 4 for each revision of the design.
