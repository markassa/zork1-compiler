"ZORK1 for
	        Zork I: The Great Underground Empire
	(c) Copyright 1983 Infocom, Inc.  All Rights Reserved."

<VERSION ZIP>

<SETG ZORK-NUMBER 1>

<SET REDEFINE T>

<OR <GASSIGNED? ZILCH>
    <SETG WBREAKS <STRING !\" !,WBREAKS>>>

<PRINC "Renovated ZORK I: The Great Underground Empire
">

<FREQUENT-WORDS?>

<INSERT-FILE "GMACROS" T>
<INSERT-FILE "GSYNTAX" T>
<INSERT-FILE "1DUNGEON" T>
<INSERT-FILE "GGLOBALS" T>

<PROPDEF SIZE 5>
<PROPDEF CAPACITY 0>
<PROPDEF VALUE 0>
<PROPDEF TVALUE 0>

<INSERT-FILE "GCLOCK" T>
<INSERT-FILE "GMAIN" T>
<INSERT-FILE "GPARSER" T>
<INSERT-FILE "GVERBS" T>
<INSERT-FILE "1ACTIONS" T>
