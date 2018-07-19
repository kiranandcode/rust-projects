### Tasks
- Support screen dragging motions 
    [x] - Setup states in dialoginputmanager 
    [X] - Create general variant to hold inputs  
    [X] - Send DialogRendererclick and motion events to event manager  
    [X] - Configure match branch to forward links to the dialoginput manager  
    [X] - Create dialog renderer message variant to represent renderwindow movement  
    [X] - Configure dialoginputmanager to send correct movements on correct state  

- Support creating boxes 
    [X] - setup states in dialog input manager
    [X] - Add button to toolbar for new mode
    [X] - Connect new button to update dialog input manager state
    [X] - Create (create new) messages for model manager 
    [X] - Connect model manager to event bus
    [X] - Configure dialog renderer to send
    
- Refactor Drawing to use cloned objects rather than mutex'd reference (speed > memory)
- Refactor drawing tasks to seperate thread then send the drawn pixbuffer home
  - add a double buffer - (current, back).
  - each time draw is called, the drawing code calls `cr.set_source_surface(0,0)` and then `cr.paint()` (i.e it does no actual calculations, but rather just paints.
    - The drawing thread has a channel to recieve `Vec<u8> buffers.
    - use `ImageSurface::create_for_data(buf, move |b| { let _ = tx.send(b); }, _, _, _, _)` for the Image surfaces - when one is dropped
  - Use `gtk_add_timeout(TIMER)` to setup a frame thread - once every 1000/30 ms, it checks if it has recieved any buffers, if it has, it then swaps the main buffer with the new buffer std::mem::swap(&mut current_buf, &mut buf), and then queues a redraw.
 - 
