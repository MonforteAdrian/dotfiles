//Modify this file to change what commands output to your statusbar, and recompile using the make command.
static const Block blocks[] = {
	/*Icon*/	/*Command*/	 	/*Update Interval*/	/*Update Signal*/
	{" 🐧 ", "~/.config/dwmblocks/scripts/kernel",		    360,		        2},

	{" 🔺 ", "~/.config/dwmblocks/scripts/upt",		        60,		            2},

	{" 📦 ", "~/.config/dwmblocks/scripts/pacupdate",		360,		        9},
	
	{" 💻 ", "~/.config/dwmblocks/scripts/memory",	        6,		            1},

	{" 🔊 ", "~/.config/dwmblocks/scripts/volume",			2,		            10},

	{" 🕑 ", "~/.config/dwmblocks/scripts/clock",			5,		            0},
};

//sets delimeter between status commands. NULL character ('\0') means no delimeter.
static char delim = '|';
