Barebones `ls` clone written in rust to learn how more about std and basic functions

Usage

minils \<params\> \<path\>

params
- "-p" => Show If ReadOnly or Writeable
- "-r" => Recursive ls
- "-l" => Show Size In Bytes
- "-s" => Sort By Size
- "-c" => Sort By Created !Partially implemented
- "-m" => Sort By Last Modified !Partially implemented
- "-d" => Show Only Directories !Unimplemented
- "-f" => Show Only Files !Unimplemented


TODO

Parse SystemTime to a human readable format

Implement show dir and file specific params

fix Display to improve readability

