# magic
### Right now It's just a home-made user interface

The idea, which brough me to start this project, is building a program which analyse a .dat file, which is the log of the played games of MTGO (magic the gathering online). So far I only built the UI. I miss any log file, so I'm not able to keep working on this. 

I would like to copy the .dat file in a folder and then use it to generate an exel file (.xml / .xlsx / .xltm), which is going to be the start point of the whole analysis. I wanna use an excell file to make it easly readable, moreover anybody may use those data to make their own analysis. 

In the end, from the excell file I would like to build another excell file which shows the analysis. That is for the same reason explained above.

As you can see it's all built in rust:
  - I used **eframe** to build the ui;
  
  ... here on I will add the crates out of the std I'm gonna use, though they are easly accessible in the Cargo.toml file: in there you can check the version of each package as well
