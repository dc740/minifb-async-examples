struct PrecalcTables
// size of tile (wall height)
this.TILE_SIZE = 64;
this.WALL_HEIGHT = 64;

// Remember that PROJECTIONPLANE = screen.  This demo assumes your screen is 320 pixels wide, 200 pixels high
this.PROJECTIONPLANEWIDTH = 320;
this.PROJECTIONPLANEHEIGHT = 200;

// We use FOV of 60 degrees.  So we use this FOV basis of the table, taking into account
// that we need to cast 320 rays (PROJECTIONPLANEWIDTH) within that 60 degree FOV.
this.ANGLE60 = this.PROJECTIONPLANEWIDTH;
// You must make sure these values are integers because we're using loopup tables.
this.ANGLE30 = Math.floor(this.ANGLE60/2);
this.ANGLE15 = Math.floor(this.ANGLE30/2);
this.ANGLE90 = Math.floor(this.ANGLE30*3);
this.ANGLE180 = Math.floor(this.ANGLE90*2);
this.ANGLE270 = Math.floor(this.ANGLE90*3);
this.ANGLE360 = Math.floor(this.ANGLE60*6);
this.ANGLE0 = 0;
this.ANGLE5 = Math.floor(this.ANGLE30/6);
this.ANGLE3 = Math.floor(this.ANGLE30/10);
this.ANGLE10 = Math.floor(this.ANGLE5*2);
this.ANGLE45 = Math.floor(this.ANGLE15*3);

// trigonometric tables (the ones with "I" such as ISiTable are "Inverse" table)
this.fSinTable=[];
this.fISinTable=[];
this.fCosTable=[];
this.fICosTable=[];
this.fTanTable=[];
this.fITanTable=[];
this.fFishTable=[];
this.fXStepTable=[];
this.fYStepTable=[];

// player's attributes
this.fPlayerX = 100;
this.fPlayerY = 160;
this.fPlayerArc = this.ANGLE60;
this.fPlayerDistanceToTheProjectionPlane = 277;
this.fPlayerHeight =32;
this.fPlayerSpeed = 16;

// Half of the screen height
this.fProjectionPlaneYCenter = this.PROJECTIONPLANEHEIGHT/2;

