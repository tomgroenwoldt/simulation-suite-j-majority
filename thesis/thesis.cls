\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{thesis}[2022/12/07 Dominik's thesis class]

\newif\ifproposal\proposalfalse
\DeclareOption{proposal}{%
\proposaltrue%
}
\ProcessOptions\relax


\ifproposal
\PassOptionsToClass{openany,chapterprefix=false}{scrbook}%
\else
\PassOptionsToClass{openright,chapterprefix=true}{scrbook}%
\fi

\LoadClass[numbers=noenddot,paper=a4,fontsize=11pt,USenglish,headsepline=0.5pt:130mm]{scrbook}
\ifproposal
\def\dedication#1{}
\def\frontmatter{}
\def\mainmatter{}
\def\backmatter{}
\let\orig@subject\subject
\def\subject#1{\orig@subject{#1 Proposal}}
\def\tableofcontents{}
\def\listoffigures{}
\def\listoftables{}
\fi

\PassOptionsToPackage{a4paper, right=40mm, left=20mm, top=40mm, bottom=40mm, headsep=10mm, footskip=20mm, headheight=10mm, nomarginpar, bindingoffset=20mm}{geometry}
\RequirePackage{geometry}
\ifproposal
\else
\setlength{\parskip}{0.25\baselineskip}
\fi
\setlength{\parindent}{1em}
\emergencystretch=1.4em
\setcounter{tocdepth}{1}

\addtokomafont{disposition}{\rmfamily}
\addtokomafont{dedication}{\itshape}

\PassOptionsToPackage{draft=false,headsepline=0.5pt:130mm}{scrlayer-scrpage}
\RequirePackage{scrlayer-scrpage}

\RequirePackage[utf8]{inputenc}
\RequirePackage[T1]{fontenc}
\RequirePackage{xpatch}
\RequirePackage{palatino}
\RequirePackage{microtype}
\RequirePackage[USenglish]{babel}
\RequirePackage{csquotes}

\RequirePackage{amsmath}
\RequirePackage{amssymb}
\RequirePackage{amsthm}

\RequirePackage{graphicx}
\RequirePackage[rgb,dvipsnames,svgnames,table]{xcolor}

\RequirePackage{booktabs}
\RequirePackage{multirow}

\RequirePackage{calc}

\RequirePackage{float}

\RequirePackage{tikz}
\usetikzlibrary{calc}
\definecolor{tuhh}{HTML}{00C1D4}%

\ifproposal
\else
\extratitle{%
\def\@dcm@subject@cmp{Bachelor's Thesis}%
\ifx\@subject\@dcm@subject@cmp%
\else%
\begin{center}%
\@title%
\end{center}%
\fi%
}
\fi
\lowertitleback{
\begin{minipage}{12cm}
\begin{flushleft}
\noindent \textbf{Hamburg University of Technology}\newline
Institute for Data Engineering

\medskip

Am Schwarzenberg-Campus 1\newline
21073 Hamburg\newline
Germany

\bigskip

\noindent\textbf{\@author}\newline
\textit{\@title}\newline
\@date{}\ifproposal\else\long\def\@dcm@commit@cmp{unknown}\ifx\commit\@dcm@commit@cmp\else~$\cdot $ commit \StrLeft{\commit}{7}\ on branch \branch\fi\fi
\end{flushleft}
\end{minipage}
}
\ifproposal
\else
\usepackage{xstring}
\IfFileExists{../.git/HEAD}{%
\usepackage{catchfile}%
\CatchFileDef{\headfull}{../.git/HEAD}{}%
\StrGobbleRight{\headfull}{1}[\head]%
\StrBehind[2]{\head}{/}[\branch]%
\IfFileExists{../.git/refs/heads/\branch}{%
    \CatchFileDef{\commit}{../.git/refs/heads/\branch}{}}{%
    \newcommand{\commit}{unknown}}%
}{%
    \newcommand{\branch}{unknown}%
    \newcommand{\commit}{unknown}%
}
\fi
% Define location field
\def\location#1{\gdef\@location{#1}}
\def\@location{}

% Define supervisors field
\ifproposal
\long\def\@dcm@s#1{\@ifnextchar\and{}{\@ifnextchar\@dcm@end{\@gobble}{\@dcm@s}}}

\long\def\supervisors#1{%
\publishers{\large\raggedright submitted to the Institute for Data Engineering\newline at the Hamburg University of Technology.

\vspace{2cm}

\begin{minipage}[t]{6cm}
\@location, \@date 
\end{minipage}\hfill
\begin{minipage}[t]{6cm}\raggedleft
\rule{6cm}{0.5pt} \\
\@author \\[1cm]
\rule{6cm}{0.5pt} \\
\begingroup
\let\@dcm@nl\\
\def\and{\@dcm@nl[1cm]\rule{6cm}{0.5pt}\@dcm@nl}
\def\@dcm@end{}
\long\def\\{\@dcm@s}
#1\@dcm@end
\endgroup
\end{minipage}%
}%
}%
\else
\def\supervisors#1{%
\publishers{\large\raggedright submitted to the Institute for Data Engineering\newline at the Hamburg University of Technology.

\vspace{2cm}
\raggedleft

\begin{minipage}[t]{6cm}
Supervisors:
\end{minipage}\hfill
\begin{minipage}[t]{6cm}\raggedleft
\def\and{\\[1cm]}
#1
\end{minipage}%
}%
}%
\fi

\titlehead{
\begin{tikzpicture}[overlay,remember picture]
    \draw [line width=1pt, color=tuhh]
        ($ (current page.north west) + (2cm,-2cm) $)
        rectangle
        ($ (current page.south east) + (-2cm,2cm) $);
    \draw [line width=1pt, color=tuhh]
        ($ (current page.south west) + (2cm,4cm) $)
        --
        ($ (current page.south east) + (-2cm,4cm) $);
    \node[inner sep=0pt,anchor=west] (logo-left) at ($(current page.south west) + (2.5cm,3cm)$)
    {\IfFileExists{tuhh-logo.pdf}{\includegraphics[height=1cm]{tuhh-logo}}{}};
    \node[inner sep=0pt,anchor=east] (logo-right) at ($(current page.south east) + (-2.5cm,3cm)$)
    {\IfFileExists{ide-logo.pdf}{\includegraphics[height=1cm]{ide-logo}}{}};

    \node[inner sep=0pt,anchor=center] (bottomtext)
    at ($(current page.south) + (0cm,2cm)$)
    {\minipage{7cm}\endminipage};

\end{tikzpicture}%
}

% Chapter Headlines
\ifproposal
\RedeclareSectionCommand[style=section]{chapter}
\else
\renewcommand*{\chapterformat}{%
\pagestyle{scrheadings}%
\color{gray} \raggedleft \bfseries\fontsize{120}{0}\selectfont\thechapter \vskip 1cm
}
\renewcommand*{\chapterheadstartvskip}{\vskip 2cm}
\renewcommand*{\chapterheadendvskip}{\vskip 1cm}
\fi
\renewcommand*{\chaptermarkformat}{%
  \thechapter\autodot\enskip%
}

% Abstract
\ifproposal
\RequirePackage{comment}
\excludecomment{preface}
\excludecomment{abstract}
\else
\newenvironment{preface}[1]{\markboth{#1}{}%
\chapter*{#1}%
\addcontentsline{toc}{chapter}{#1}}
{\cleardoublepage}

\newenvironment{abstract}{\preface{Abstract}}{\endpreface}
\fi

% Biblatex
\PassOptionsToPackage{style=alphabetic,
maxbibnames=99,
maxcitenames=2,
mincitenames=1,
maxalphanames=4,
minalphanames=3,
date=year,
sorting=nyt,
bibencoding=auto,
backend=biber}{biblatex}
\RequirePackage{biblatex}
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%55
%% Fix up bibliography
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%55
% clear quotes around the title in \citetitle{}, but use italics
\DeclareFieldFormat*{citetitle}{\itshape #1}
\DeclareFieldFormat*{title}{\itshape #1\isdot}
\renewcommand*{\intitlepunct}{\space}

%%%%%
\DeclareFieldFormat[inproceedings]{volume}{Volume #1}
\DeclareFieldFormat*{pages}{pages #1}
%\DeclareFieldFormat*{volume}{Volume #1}
%\DeclareFieldFormat*{number}{\mkbibparens{#1}}
%
%\renewbibmacro*{volume+number+eid}{%
%  \setunit{\addcomma\space}
%  \printfield{volume}%
%  \setunit*{\addnbspace}% NEW (optional); there's also \addnbthinspace
%  \printfield{number}%
%}

%\renewbibmacro*{issue+date}{%
%  \setunit{\addcomma\space}
%  %\printtext[parens]{%
%    \iffieldundef{issue}
%      {\usebibmacro{date}}
%      {\printfield{issue}%
%       \setunit*{\addspace}%
%       \usebibmacro{date}}%}%
%  \newunit}
%%%%%
\AtEveryBibitem{% Clean up the bibtex rather than editing it
 % Make a volume number actually be the series number for inproceedings
 \ifentrytype{inproceedings}{%
  \iffieldundef{series}{}{%
    \savefield{volume}{\theoriginalvolume}%
    \restorefield{number}{\theoriginalvolume}%
    \clearfield{volume}%
  }%
 }{}%
 % and same for book
 \ifentrytype{book}{%
  \iffieldundef{series}{}{%
    \savefield{volume}{\theoriginalvolume}%
    \restorefield{number}{\theoriginalvolume}%
    \clearfield{volume}%
  }%
 }{}%
 % And remove address, location, and editor
 \clearlist{address}%
 %\clearfield{date}%
 %\clearfield{eprint}%
 %\clearfield{isbn}%
 %\clearfield{issn}%
 \clearlist{location}%
 %\clearfield{month}%
 %\clearfield{series}%
 \ifentrytype{book}{}{% Remove publisher and editor except for books
  %\clearlist{publisher}%
  \clearname{editor}%
 }%
}
\AtEveryCitekey{\UseBibitemHook}
%%%%%

\renewcommand*{\multicitedelim}{\addcomma\space}
\renewcommand{\labelnamepunct}{\addcolon\space}

\renewbibmacro*{issue+date}{%
  \setunit{\addcomma\space}
  %\printtext[parens]{%
    \iffieldundef{issue}
      {\usebibmacro{date}}
      {\printfield{issue}%
       \setunit*{\addspace}%
       \usebibmacro{date}}%}%
  \newunit}


\renewbibmacro*{publisher+location+date}{%
  \printlist{location}%
  \iflistundef{publisher}
    {\setunit*{\addcomma\space}}
    {\setunit*{\addcolon\space}}%
  \printlist{publisher}%
  \setunit*{\addcomma\space}%
  \addcomma\space\usebibmacro{date}%
  \newunit}



\DeclareFieldFormat*{number}{\mkbibparens{#1}}
\DeclareFieldFormat[inproceedings]{number}{\addcomma\space volume #1\addcomma}
%\DeclareFieldFormat[inproceedings]{number}{\mkbibparens{#1}\addcomma}
\DeclareFieldFormat[incollection]{volume}{\addcomma\space volume #1\addcomma}
\DeclareFieldFormat[article]{volume}{volume #1}

\renewbibmacro*{volume+number+eid}{%
  \setunit{\addcomma\space}
  \printfield{volume}%
  \setunit*{\addnbthinspace}% NEW (optional); there's also \addnbthinspace
  \printfield{number}%
}

\xpatchbibdriver{incollection}{%
  \usebibmacro{maintitle+booktitle}%
  \newunit\newblock
}{%
  \usebibmacro{maintitle+booktitle}%
  \iffieldundef{volume}{
    \newunit\newblock
  }{
    \addcomma\space\newblock
  }
}{}{}

%use comma before edition for books
\xpatchbibdriver{book}{%
\newunit\newblock
\printfield{edition}%
}
{%
\setunit{\addcomma\space}\newblock
\printfield{edition}%
}{}{}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\ifproposal
\def\insertdeclaration{}
\else
\def\insertdeclaration{
\chapter*{Eidesstattliche Erklärung}
\thispagestyle{empty}

Hiermit versichere ich an Eides statt, dass die vorliegende Arbeit selbstständig verfasst wurde und keine anderen als die angegebenen Quellen und Hilfsmittel benutzt wurden.

\vspace{1.5cm}

\noindent \@location, am \@date \hfill \@author
}
\fi
