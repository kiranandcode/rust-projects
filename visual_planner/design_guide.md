## Design Guides
- Allow for representing conversations as a sequence of pieces of speach
    - Should support annotating (pieces of speach) with a character
- Should provide conversations started from trigger points
- Should allow conversations to be started on entering a scene
- Should support an inventory system 
- Should support branching conversation paths
- Should place conversations within the context of rooms
    - Should support multiple states for a room



## Components
- Text Boxes []
    - Can be annotated with characters
- Decision Boxes < >
    - Provide condition
- Variable Boxes ()
    - Connect to variables
- State change boxes _ _
    - specify a state to change to
- Entry points *
    - Can be guarded with a states


## GUI Mockup
It sucks because I had to do it using ASCII-art


```                  
                      Map
                     Editor        Node
               Dialog tab         Editor 
               Editor  |           tab  Variable
  Map listing   tab    |            |   Editor tab
       |         |     |            |    |
       v         V     v            V    v
   __________ _----___----_______ _----_---_
   | Map1   ||                  ||          |
   | Map2   || |---|            || prop1  []|
   |        || |___|            || name  __ |
   |        ||   |      |---|   ||          |
   |        ||   |_____>|___|   ||          |
   |        ||            |     ||          |
   |________||           (x)    ||          |
   |[] lay0 ||            |     ||          |
   |________||____________V_____||__________|
      ^
      |
   Layer select


```

## System Design


```
        Tmp object for representing
        changes in motion, which are
        then sent via event to the mnger
              |      _____________________________________ this is all model
              V     |           ________________
  |-----|  |-----|   |-----|    | Undo Manager |
  |rend.|  |tmpmn|   |mnger|    |              |
  |_____|  |_____|   |_____|    |______________|
     |        |         |             _____        
    |_|----->|_| ----->|_|            |   |  <- keeps track of object id, and state of object
    |_|----->|_| ----->|_|            |___|     
    |_|----->|_| ----->|_|              |         
    |_|----->|_| ----->|_|            __|__          
    |_|----->|_| ----->|_|            |   |         
    |_|----->|_| ----->|_|            |___|        
    |_|      |_|                                   
   Render    Temp     Object                      
   Queue    Object     List                         
    ^   ____________________   |--------|
    |  |____________________|  |Evnt Mng|  <- evnt manager runs in a seperate thread, handling
    |                          |________|     events and passing them to their recipient
  this                      
   is        _________
   view     |  Input  |
            | Manager |
            |_________|
                ^
                | I/O
                |

```
