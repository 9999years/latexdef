Imagine this but with pretty colors:

    $ latexdef def @newcommand author box parskip thanks begin bedgin
    \def is primitive.
    \@newcommand = #1[#2] -> \kernel@ifnextchar [{\@xargdef #1[#2]}{\@argdef #1[#2]}
    \author = #1 -> \gdef \@author {#1}
    \box is primitive.
    \parskip is primitive.
    \thanks = #1 -> \footnotemark \protected@xdef \@thanks {\@thanks \protect \footnotetext [\the \c@footnote ]{#1}}
    \begin = #1 -> \@ifundefined {#1}{\def \reserved@a {\@latex@error {Environment #1 undefined}\@eha }}{\def \reserved@a {\def \@currenvir {#1}\edef \@currenvline {\on@line }\csname #1\endcsname }}\@ignorefalse \begingroup \@endpefalse \reserved@a

Further:

    $ latexdef --help
    latexdef 0.1.0
    Rebecca Turner <rbt@sent.as>
    Prints definitions of LaTeX macros.

    USAGE:
        latexdef [FLAGS] [OPTIONS] <COMMAND>... --documentclass <CLASS> --engine <ENGINE>

    FLAGS:
        -e, --expl3      Enable LaTeX3e features with the expl3 package
            --math       Load common math packages (amsmath, amssymb, amsthm, mathtools)
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
            --documentclass <CLASS>    Document class to use [default: article]
            --engine <ENGINE>          TeX engine to run. [default: latex]
        -p, --packages <PACKAGE>...    Packages to load

    ARGS:
        <COMMAND>...    Commands to show definitions of
