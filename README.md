Imagine this but with pretty colors:

    $ latexdef def @newcommand author box parskip thanks begin bedgin
    \def is primitive.
    \@newcommand = #1[#2] -> \kernel@ifnextchar [{\@xargdef #1[#2]}{\@argdef #1[#2]}
    \author = #1 -> \gdef \@author {#1}
    \box is primitive.
    \parskip is primitive.
    \thanks = #1 -> \footnotemark \protected@xdef \@thanks {\@thanks \protect \footnotetext [\the \c@footnote ]{#1}}
    \begin = #1 -> \@ifundefined {#1}{\def \reserved@a {\@latex@error {Environment #1 undefined}\@eha }}{\def \reserved@a {\def \@currenvir {#1}\edef \@currenvline {\on@line }\csname #1\endcsname }}\@ignorefalse \begingroup \@endpefalse \reserved@a
