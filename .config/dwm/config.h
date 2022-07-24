/* appearance */
static const unsigned int borderpx  = 2;        /* border pixel of windows */
static const unsigned int gappx     = 4;        /* gaps between windows */
static const unsigned int snap      = 32;       /* snap pixel */
static const int showbar            = 1;        /* 0 means no bar */
static const int topbar             = 1;        /* 0 means bottom bar */
static const int horizpadbar        = 6;        /* horizontal padding for statusbar */
static const int vertpadbar         = 11;        /* vertical padding for statusbar */

static const char *fonts[]     = {"JetBrains Nerd Font Mono:weight=Regular:size=11:antialias=true:autohint=true",
                                  "SauceCodePro Nerd Font Mono:size=18"
};

static const char col_bg[]       = "#282A36";
static const char col_fg[]       = "#F8F8F2";
static const char col_purple[]   = "#BD93F9";
static const char col_green[]    = "#50FA7B";
static const char col_cyan[]     = "#8BE9FD";

static const char *colors[][3]   = {
  /*               fg         bg         border   */
  [SchemeNorm] = { col_cyan, col_bg, col_bg },
  [SchemeSel]  = { col_purple, col_bg,  col_purple },
  [SchemeOcc]  = { col_purple, col_bg, col_bg },
  [SchemeTitle]  = { col_fg, col_bg,  col_bg  },
};

/* tagging */
// static const char *tags[] = { "1", "2", "3", "4", "5", "6", "7", "8", "9" };
static const char *tags[] = { "", "", "", "", "", "", "", "", "" };

static const unsigned int ulinepad	= 3;  /* horizontal padding between the underline and tag */
static const unsigned int ulinestroke	= 2;  /* thickness / height of the underline */
static const unsigned int ulinevoffset	= 0;  /* how far above the bottom of the bar the line should appear */
static const int ulineall 		= 0;  /* 1 to show underline on all tags, 0 for just the active ones */

static const Rule rules[] = {
  /* xprop(1):
   *	WM_CLASS(STRING) = instance, class
   *	WM_NAME(STRING) = title
   */
  /* class      instance    title       tags mask     isfloating   monitor */
  { "Google",     NULL,       NULL,       1,            0,           1 },
};

/* layout(s) */
static const float mfact     = 0.50; /* factor of master area size [0.05..0.95] */
static const int nmaster     = 1;    /* number of clients in master area */
static const int resizehints = 0;    /* 1 means respect size hints in tiled resizals */
static const int lockfullscreen = 1; /* 1 will force focus on the fullscreen window */

static const Layout layouts[] = {
  /* symbol     arrange function */
  { "[]=",      tile },    /* first entry is default */
  { "><>",      NULL },    /* no layout function means floating behavior */
  { "[M]",      monocle },
};

/* key definitions */
#define MODKEY ControlMask
#define TAGKEYS(KEY,TAG) \
  { MODKEY,                       KEY,      view,           {.ui = 1 << TAG} }, \
  { MODKEY|ShiftMask,             KEY,      tag,            {.ui = 1 << TAG} }, \

/* helper for spawning shell commands in the pre dwm-5.0 fashion */
#define SHCMD(cmd) { .v = (const char*[]){ "/bin/sh", "-c", cmd, NULL } }

/* commands */
static char dmenumon[2] = "0"; /* component of dmenucmd, manipulated in spawn() */
static const char *dmenucmd[] = { "dmenu_run", "-p", "Run: ", NULL };
static const char *termcmd[]  = { "st", NULL };
static const char *chromecmd[]  = { "google-chrome-stable", NULL };

static Key keys[] = {
  /* modifier                     key        function        argument */
  { MODKEY,                       XK_p,      spawn,          {.v = dmenucmd } },
  { MODKEY,                       XK_Return, spawn,          {.v = termcmd } },
  { MODKEY,                       XK_b,      spawn,          {.v = chromecmd } },
  { MODKEY,                       XK_j,      focusstack,     {.i = +1 } },
  { MODKEY,                       XK_k,      focusstack,     {.i = -1 } },
  { MODKEY|ShiftMask,             XK_i,      incnmaster,     {.i = +1 } },
  { MODKEY|ShiftMask,             XK_d,      incnmaster,     {.i = -1 } },
  { MODKEY|ShiftMask,             XK_h,      setmfact,       {.f = -0.05} },
  { MODKEY|ShiftMask,             XK_l,      setmfact,       {.f = +0.05} },

  { MODKEY|ShiftMask,             XK_t,      setlayout,      {.v = &layouts[0]} },
  { MODKEY|ShiftMask,             XK_f,      setlayout,      {.v = &layouts[1]} },
  { MODKEY|ShiftMask,             XK_m,      setlayout,      {.v = &layouts[2]} },

  { MODKEY,                       XK_comma,  focusmon,       {.i = -1 } },
  { MODKEY,                       XK_period, focusmon,       {.i = +1 } },
  { MODKEY|ShiftMask,             XK_comma,  tagmon,         {.i = -1 } },
  { MODKEY|ShiftMask,             XK_period, tagmon,         {.i = +1 } },
  TAGKEYS(                        XK_1,                      0)
  TAGKEYS(                        XK_2,                      1)
  TAGKEYS(                        XK_3,                      2)
  TAGKEYS(                        XK_4,                      3)
  TAGKEYS(                        XK_5,                      4)
  TAGKEYS(                        XK_6,                      5)
  TAGKEYS(                        XK_7,                      6)
  TAGKEYS(                        XK_8,                      7)
  TAGKEYS(                        XK_9,                      8)
  { MODKEY,                       XK_q,      killclient,     {0} },
  { MODKEY|ShiftMask,             XK_r,      quit,           {1} },
  { MODKEY|ShiftMask,             XK_q,      quit,           {0} },
};

/* button definitions */
/* click can be ClkTagBar, ClkLtSymbol, ClkStatusText, ClkWinTitle, ClkClientWin, or ClkRootWin */
static Button buttons[] = {
  /* click                event mask      button          function        argument */
  { ClkLtSymbol,          0,              Button1,        setlayout,      {0} },
  { ClkLtSymbol,          0,              Button3,        setlayout,      {.v = &layouts[2]} },
  { ClkWinTitle,          0,              Button2,        zoom,           {0} },
  { ClkStatusText,        0,              Button2,        spawn,          {.v = termcmd } },
  { ClkClientWin,         MODKEY,         Button1,        movemouse,      {0} },
  { ClkClientWin,         MODKEY,         Button2,        togglefloating, {0} },
  { ClkClientWin,         MODKEY,         Button3,        resizemouse,    {0} },
  { ClkTagBar,            0,              Button1,        view,           {0} },
  { ClkTagBar,            0,              Button3,        toggleview,     {0} },
  { ClkTagBar,            MODKEY,         Button1,        tag,            {0} },
  { ClkTagBar,            MODKEY,         Button3,        toggletag,      {0} },
};

