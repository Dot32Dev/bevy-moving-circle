> This project is currently undergoing a migration from Bevy 0.7 to Bevy 0.10. Much will be broken during this time.

# bevy-moving-circle
A yellow circle player controlled with arrow keys/WASD

<img width="1392" alt="Screen Shot 2022-03-02 at 19 51 18" src="https://user-images.githubusercontent.com/61964090/156356580-6790c24e-8311-4f46-9851-0e8d8510e1a3.png">

-------

Now you can click to spawn orange cicles lmao

![orange circles](https://user-images.githubusercontent.com/61964090/156549764-9e1d14f2-c470-41de-8bb4-f180300b2d45.gif)

(intense action packed gameplay)
how fast can you lag your computer from too many circles 😎

-------

Proper shooting now implemented: found out how to spawn entities with components that hold values, bullets now store their direction when placed and have an update system to move them every frame.

![shooting2](https://user-images.githubusercontent.com/61964090/159148223-90061417-b1b5-4fef-841b-68e9f3a1c8c1.gif)

-------

Nicer colours, fixed update, and a rotating turret that faces the player!

![ezgif-3-4c43aa534a](https://user-images.githubusercontent.com/61964090/162556963-b89d8634-231e-4d81-9646-fe7e940326c1.gif)

-------

Added my intro (which i made as a rust library, [see repo here](https://github.com/Dot32IsCool/dot32-intro-rs) <br>
Installation was as simple as
```toml
[Dependencies]
dot32_intro = { git = "https://github.com/Dot32IsCool/dot32-intro-rs"}
```
![image of intro animation](https://user-images.githubusercontent.com/61964090/168785042-728b8934-35aa-4af1-9c49-8634f00d8ce3.gif)

-------

Added AI and healthbars!

<img width="912" alt="Screen Shot 2022-08-01 at 18 34 08" src="https://user-images.githubusercontent.com/61964090/182130151-94b090f9-8a92-4760-9c33-4ef8cfa5cf62.png">
