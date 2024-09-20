Barebones `ls` clone written in rust to learn how more about std and basic functions

Usage

minils \<params\> \<path\>

params

Both types as path
- "-p" => Show If ReadOnly or Writeable
- "-l" => Show Size In Bytes

Dir path only
- "-r" => Recursive ls
- "-d" => Show Only Directories 
- "-f" => Show Only Files 
- "-s" => Sort By Size
- "-c" => Sort By Created !Partially implemented
- "-m" => Sort By Last Modified !Partially implemented


TODO

Parse SystemTime to a human readable format

fix Display to improve readability

