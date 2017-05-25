<div class="Section1">

<span style="font-size:20.0pt">BeerXML</span>

<span style="font-size:20.0pt">An XML Standard</span>

**<span style="font-size:20.0pt">for Beer Brewing Data</span>**

**<span style="font-size:20.0pt">Version 1.0</span>**

****

 

**Created by:**

**Brad Smith – “BeerSmith”**

**Drew Avis – “Strangebrew”**

**Michael Taylor – “SUDS”**

**Andrew Perron – “DrewBrew”**

**David Johnson – “QBrew”**

****

 

****

 

<span style="font-size:16.0pt;mso-bidi-font-size:12.0pt">Purpose</span>
=======================================================================

The primary purpose for the standard is the exchange of recipes, but it
could also be used for the exchange of other brewing data.<span
style="mso-spacerun:yes">  </span>For example a table of hops could be
exported as a series of XML hop records in a single file.

 

The optional appendix adds tags for use in the display of brewing data
using XML style sheets or XML compatible report generators.<span
style="mso-spacerun:yes">  </span>As the tags in the appendix are for
display only and may include rounded values. <span
style="mso-spacerun:yes"> </span>We do not recommend relying on any of
these tags for data import.

 

General
-------

Brewing data will follow the XML standard as a basis.<span
style="mso-spacerun:yes">  </span>To be compliant the program must be
able to import or export the required tags, recognize the data formats
and units, and follow basic XML conventions.<span
style="mso-spacerun:yes">  </span>In addition the program may support
optional tags that have “No” in the Required column.

 

For simplicity, the convention of using a separate tag for each data
entry as in the following will be used:

&lt;HOP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Cascade&lt;/NAME&gt;

&lt;/HOP&gt;

 

Though equivalent, the following XML format (i.e. XML Attributes) should
**NOT** be used.

 

&lt;HOP NAME="Cascade"&gt; &lt;/HOP&gt;

 

Each new tag will be put on a separate line, with the start and end of
the tag surrounding the data.<span style="mso-spacerun:yes"> 
</span>Tags starting and ending a record will be placed on their own
line (see examples).

 

File Extension
--------------

The file extension “.xml” should be used for all BeerXML files.<span
style="mso-spacerun:yes">  </span>For example, a recipe file might be
named “recipes.xml”.

 

Comments
--------

Comments may be embedded per the XML standard, but all comments shall be
ignored by importing programs.

 

**Sample XML comment**

&lt;!-- This is a comment line in the XML format<span
style="mso-spacerun:yes">  </span>--&gt;

 

Special Characters
------------------

The exporting and importing programs should recognize and translate the
normal XML special character codes if they appear in any of the data
strings.<span style="mso-spacerun:yes">  </span>These include:

 

<div align="center">

+--------------------------------------+--------------------------------------+
| ***Character***                      | ***XML Code***                       |
+--------------------------------------+--------------------------------------+
| &                                    | &amp;                                |
+--------------------------------------+--------------------------------------+
| &lt;                                 | &lt;                                 |
|                                      |                                      |
|                                      |                                      |
+--------------------------------------+--------------------------------------+
| &gt;                                 | &gt;                                 |
|                                      |                                      |
|                                      |                                      |
+--------------------------------------+--------------------------------------+
| “                                    | &quot;                               |
+--------------------------------------+--------------------------------------+
| ‘                                    | &apos;                               |
+--------------------------------------+--------------------------------------+

</div>

 

XML Header
----------

Per the XML standard, all files should begin with the following header
line as the first line.<span style="mso-spacerun:yes">  </span>After the
XML header a record set should start (for example
&lt;RECIPES&gt;…&lt;/RECIPES&gt; or &lt;HOPS&gt; … &lt;/HOPS&gt;).

<span style="font-size:9.5pt;color:black"></span>

 

**Required XML Header Example with Recipes tag:**

****

 

<span style="color:black">&lt;?xml version="1.0"
encoding="ISO-8859-1"?&gt;</span>

<span style="color:black">&lt;RECIPES&gt;</span>

<span style="color:black"><span style="mso-spacerun:yes">   </span>…
</span>

<span style="color:black">&lt;/RECIPES&gt;</span>

 

Tag Names
---------

Tag names will be uppercase.<span style="mso-spacerun:yes">  </span>For
example "HOP" is acceptable, but "hop" and Hop" are not.

 

Version
-------

All records have a required &lt;VERSION&gt; tag that denotes the version
of the XML standard.<span style="mso-spacerun:yes">  </span>All should
be set to the integer 1 for this version of the standard.<span
style="mso-spacerun:yes">  </span>It is our intent that future versions
of the standard will be backward compatible with the older versions, but
the VERSION tag allows newer programs to check for a higher version of
the standard or do conversions if required to be backward compatible.

 

Non-Standard Tags
-----------------

Per the XML standard, all non-standard tags will be ignored by the
importing program.<span style="mso-spacerun:yes">  </span>This allows
programs to store additional information if desired using their own
tags.<span style="mso-spacerun:yes">  </span>Any tags not defined as
part of this standard may safely be ignored by the importing program.

 

Data Formats
------------

**- Record Set –** A special tag that starts a particular set of
data.<span style="mso-spacerun:yes">  </span>For example an XML table
that consists of a set of hops records might start with a &lt;HOPS&gt;
tag to denote that this is the start of hops records.<span
style="mso-spacerun:yes">  </span>After the last record, a &lt;/HOPS&gt;
tag would be used.

 

**- Record -** Denotes a tag that starts or ends a particular record --
for example "HOP" might start a hops record or "FERMENTABLE" might start
a fermentable record.

 

**- Percentage -** Denotes a percentage - all percentages are expressed
as percent out of 100- for example 10.4% is written as "10.4" and not
"0.104"

 

- **List -** The data has only a fixed number of values that are
selected from the list in the description table for the tag.<span
style="mso-spacerun:yes">  </span>These items are case sensitive, and no
other values are allowed.

 

- **Text -** The data is free format text.<span
style="mso-spacerun:yes">  </span>For multiline entries, line breaks
will be preserved where possible and the text may be truncated on import
if the text is too long for the importing program to store.<span
style="mso-spacerun:yes">  </span>Multiline entries may be split with
either a newline (Unix format) or a carriage return – newline
combination (DOS format).<span style="mso-spacerun:yes"> 
</span>Importing programs should accept either.

 

**- Boolean -** May be either TRUE or FALSE, with TRUE and FALSE in
capitals.<span style="mso-spacerun:yes">  </span>A default value should
be specified for optional fields - the default is used if the value is
not present.

 

**- Integer -** An integer number with no decimal point.<span
style="mso-spacerun:yes">  </span>May include negative values - examples
include ...-3, -2, -1, 0, 1, 2, 3,...

 

- **Floating Point -** A floating point number, usually expressed in its
simplest form with a decimal point as in "1.2", "0.004", etc...<span
style="mso-spacerun:yes">  </span>Programs shall endeavor to store as
many significant digits as possible to avoid truncating or losing small
values.

 

Units
-----

For this portion of the standard ALL units must be fixed.<span
style="mso-spacerun:yes">  </span>It is the responsibility of the
importing or exporting program to convert to and from the units below if
needed.

### Weight Units

All weights will be measured in Kilograms (kg).<span
style="mso-spacerun:yes">  </span>For small values the exporting program
will make an effort to preserve as many significant digits as possible.

### Volume Units

<span style="mso-bidi-font-weight:bold">All volumes will be measured in
Liters (l).<span style="mso-spacerun:yes">  </span></span>For small
values the exporting program will make an effort to preserve as many
significant digits as possible.

### Temperature Units

<span style="mso-bidi-font-weight:bold">All temperatures will be
measured in degrees Celsius.</span>

### Time Units

<span style="mso-bidi-font-weight:bold">All times will be in minutes or
fractions thereof – unless otherwise specified in the tag
description.</span>

### Specific Gravity Units

<span style="mso-bidi-font-weight:bold">Specific Gravity will be
measured relative to the weight of the same size sample of water. For
example “1.035”, “1.060”, etc…</span>

### Pressure Units

<span style="mso-bidi-font-weight:bold">Pressures will be measured in
kilopascals (kPa)</span>

 

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Record Sets</span>

The following special tags are used to denote a set of records.<span
style="mso-spacerun:yes">  </span>This allows more than one record of a
single type to be embedded within a recipe, and also allows separate XML
tables to be exported and imported.<span style="mso-spacerun:yes"> 
</span>For example a standalone collection of hops records might be
exported as a “HOPS” table by starting the table with &lt;HOPS&gt;,
continuing with a number of hops records and ending the table with
&lt;/HOPS&gt;

 

In a recipe, these record set identifiers are also used to separate
records of different types.<span style="mso-spacerun:yes">  </span>For
example, all HOP records used in a recipe will be enclosed between
&lt;HOPS&gt;…&lt;/HOPS&gt; identifiers.

 

 

+--------------------------+--------------------------+--------------------------+
| **Data tag**             | Format                   | **Description**          |
|                          | ======                   |                          |
+--------------------------+--------------------------+--------------------------+
| **HOPS**                 | Record Set               | Encloses a set of one or |
|                          |                          | more Hop records.        |
+--------------------------+--------------------------+--------------------------+
| **FERMENTABLES**         | Record Set               | Encloses a set of one or |
|                          |                          | more Fermentable         |
|                          |                          | records.                 |
+--------------------------+--------------------------+--------------------------+
| **YEASTS**               | Record Set               | Encloses a set of one or |
|                          |                          | more Yeast records.      |
+--------------------------+--------------------------+--------------------------+
| **MISCS**                | Record Set               | Encloses a set of one or |
|                          |                          | more Misc records        |
+--------------------------+--------------------------+--------------------------+
| **WATERS**               | Record Set               | Encloses a set of one or |
|                          |                          | more Water records       |
+--------------------------+--------------------------+--------------------------+
| **STYLES**               | Record Set               | Encloses a set of one or |
|                          |                          | more Beer Styles         |
+--------------------------+--------------------------+--------------------------+
| **MASH\_STEPS**          | Record Set               | Used within a MASH       |
|                          |                          | profile to record the    |
|                          |                          | steps                    |
+--------------------------+--------------------------+--------------------------+
| **MASHS**                | Record Set               | Used for a set of one or |
|                          |                          | more mash profiles       |
+--------------------------+--------------------------+--------------------------+
| **RECIPES**              | Record Set               | Encloses one or more     |
|                          |                          | recipe records.          |
+--------------------------+--------------------------+--------------------------+
| **EQUIPMENTS**           | Record Set               | Set of one or more       |
|                          |                          | equipment records.       |
+--------------------------+--------------------------+--------------------------+

 

 

**Example: A set of 2 hops**

&lt;HOPS&gt;

<span style="mso-spacerun:yes">  </span>&lt;HOP&gt;

<span style="mso-spacerun:yes">   </span>&lt;!--<span
style="mso-spacerun:yes">  </span>hop 1 fields here --&gt;

<span style="mso-spacerun:yes">  </span>&lt;/HOP&gt;

<span style="mso-spacerun:yes">  </span>&lt;HOP&gt;

<span style="mso-spacerun:yes">   </span>&lt;!--<span
style="mso-spacerun:yes">  </span>hop 2 fields here --&gt;

<span style="mso-spacerun:yes">   </span>&lt;/HOP&gt;

&lt;/HOPS&gt;

#### 

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Hops</span>

The “HOP” identifier is used to define all varieties of hops.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **HOP**            | Yes                | Record             | Starts a hops      |
|                    |                    |                    | ingredient record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;HOP&gt;….&lt;/ |
|                    |                    |                    | HOP&gt;            |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the hops will   |
|                    |                    |                    | be ignored.        |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the hops   |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Should be set to 1 |
|                    |                    |                    | for this version   |
|                    |                    |                    | of the XML         |
|                    |                    |                    | standard.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be a    |
|                    |                    |                    | higher number for  |
|                    |                    |                    | later versions but |
|                    |                    |                    | all later versions |
|                    |                    |                    | shall be backward  |
|                    |                    |                    | compatible.        |
+--------------------+--------------------+--------------------+--------------------+
| **ALPHA**          | Yes                | Percentage         | Percent alpha of   |
|                    |                    |                    | hops - for example |
|                    |                    |                    | "5.5" represents   |
|                    |                    |                    | 5.5% alpha         |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT**         | Yes                | Weight (kg)        | Weight in          |
|                    |                    |                    | Kilograms of the   |
|                    |                    |                    | hops used in the   |
|                    |                    |                    | recipe.            |
+--------------------+--------------------+--------------------+--------------------+
| **USE**            | Yes                | List               | May be "Boil",     |
|                    |                    |                    | "Dry Hop", "Mash", |
|                    |                    |                    | "First Wort" or    |
|                    |                    |                    | "Aroma".<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Note that   |
|                    |                    |                    | "Aroma" and "Dry   |
|                    |                    |                    | Hop" do not        |
|                    |                    |                    | contribute to the  |
|                    |                    |                    | bitterness of the  |
|                    |                    |                    | beer while the     |
|                    |                    |                    | others do.<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Aroma hops  |
|                    |                    |                    | are added after    |
|                    |                    |                    | the boil and do    |
|                    |                    |                    | not contribute     |
|                    |                    |                    | substantially to   |
|                    |                    |                    | beer bitterness.   |
+--------------------+--------------------+--------------------+--------------------+
| **TIME**           | Yes                | Time (min)         | The time as        |
|                    |                    |                    | measured in        |
|                    |                    |                    | minutes.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Meaning is  |
|                    |                    |                    | dependent on the   |
|                    |                    |                    | “USE” field.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For “Boil”  |
|                    |                    |                    | this is the boil   |
|                    |                    |                    | time.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For “Mash”  |
|                    |                    |                    | this is the mash   |
|                    |                    |                    | time.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For “First  |
|                    |                    |                    | Wort” this is the  |
|                    |                    |                    | boil time.<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For “Aroma” |
|                    |                    |                    | this is the steep  |
|                    |                    |                    | time.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For “Dry    |
|                    |                    |                    | Hop” this is the   |
|                    |                    |                    | amount of time to  |
|                    |                    |                    | dry hop.           |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Textual notes      |
|                    |                    |                    | about the hops,    |
|                    |                    |                    | usage,             |
|                    |                    |                    | substitutes.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be a    |
|                    |                    |                    | multiline entry.   |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | No                 | List               | May be             |
|                    |                    |                    | "Bittering",       |
|                    |                    |                    | "Aroma" or "Both"  |
+--------------------+--------------------+--------------------+--------------------+
| **FORM**           | No                 | List               | May be "Pellet",   |
|                    |                    |                    | "Plug" or "Leaf"   |
+--------------------+--------------------+--------------------+--------------------+
| **BETA**           | No                 | Percentage         | Hop beta           |
|                    |                    |                    | percentage - for   |
|                    |                    |                    | example "4.4"      |
|                    |                    |                    | denotes 4.4 % beta |
+--------------------+--------------------+--------------------+--------------------+
| **HSI**            | No                 | Percentage         | Hop Stability      |
|                    |                    |                    | Index - defined as |
|                    |                    |                    | the percentage of  |
|                    |                    |                    | hop alpha lost in  |
|                    |                    |                    | 6 months of        |
|                    |                    |                    | storage            |
+--------------------+--------------------+--------------------+--------------------+
| **ORIGIN**         | No                 | Text               | Place of origin    |
|                    |                    |                    | for the hops       |
+--------------------+--------------------+--------------------+--------------------+
| **SUBSTITUTES**    | No                 | Text               | Substitutes that   |
|                    |                    |                    | can be used for    |
|                    |                    |                    | this hops          |
+--------------------+--------------------+--------------------+--------------------+
| **HUMULENE**       | No                 | Percent            | Humulene level in  |
|                    |                    |                    | percent.           |
+--------------------+--------------------+--------------------+--------------------+
| **CARYOPHYLLENE**  | No                 | Percent            | Caryophyllene      |
|                    |                    |                    | level in percent.  |
+--------------------+--------------------+--------------------+--------------------+
| **COHUMULONE**     | No                 | Percent            | Cohumulone level   |
|                    |                    |                    | in percent         |
+--------------------+--------------------+--------------------+--------------------+
| **MYRCENE**        | No                 | Percent            | Myrcene level in   |
|                    |                    |                    | percent            |
+--------------------+--------------------+--------------------+--------------------+

 

**Example with required fields only:**

 

&lt;HOP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Cascade&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;ALPHA&gt;5.0&lt;/ALPHA&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.100&lt;/AMOUNT&gt;

&lt;USE&gt;Boil&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;TIME&gt;60&lt;/TIME&gt;

&lt;/HOP&gt;

 

**Example dry hop for three days:**

****

 

&lt;HOP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Fuggles&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;ALPHA&gt;4.5&lt;/ALPHA&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.250&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;USE&gt;Dry Hop&lt;/USE&gt;

<span style="mso-spacerun:yes">  </span>&lt;TIME&gt;10080.0&lt;/TIME&gt;

&lt;/HOP&gt;

<span style="mso-spacerun:yes"> </span>

**Example Mash Hops with All Fields - in shuffled order (acceptable):**

&lt;HOP&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.050&lt;/AMOUNT&gt;

&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;USE&gt;Mash&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;ALPHA&gt;4.5&lt;/ALPHA&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt; This hop is a
really cool hops - you can use it for anything.

It leaps over buildings in a single bound, is faster than

a speeding bullet and makes really bitter beer.

&lt;/NOTES&gt;

&lt;TIME&gt;45.0&lt;/TIME&gt;

&lt;BETA&gt;5.5 &lt;/BETA&gt;

&lt;NAME&gt;Super Hops&lt;/NAME&gt;

&lt;ORIGIN&gt;Planet Krypton&lt;/ORIGIN&gt;

&lt;SUBSTITUTES&gt;Goldings, Fuggles, Super Alpha&lt;/SUBSTITUTES&gt;

&lt;MYRCENE&gt;24.4&lt;/MYRCENE&gt;

&lt;HSI&gt;30&lt;/HSI&gt;

&lt;FORM&gt;Leaf&lt;/FORM&gt;

&lt;TYPE&gt;Bittering&lt;/TYPE&gt;

&lt;COHUMULONE&gt;13.2&lt;/COHUMULONE&gt;

&lt;/HOP&gt;

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Fermentable</span>

The term "fermentable" encompasses all fermentable items that contribute
substantially to the beer including extracts, grains, sugars, honey,
fruits.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **FERMENTABLE**    | Yes                | Record             | Starts a           |
|                    |                    |                    | fermentable        |
|                    |                    |                    | ingredient record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;FERMENTABLE&gt |
|                    |                    |                    | ;….                |
|                    |                    |                    | &lt;/FERMENTABLE&g |
|                    |                    |                    | t;                 |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the fermentable |
|                    |                    |                    | will be ignored.   |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the        |
|                    |                    |                    | fermentable.       |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Should be set to 1 |
|                    |                    |                    | for this version   |
|                    |                    |                    | of the XML         |
|                    |                    |                    | standard.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be a    |
|                    |                    |                    | higher number for  |
|                    |                    |                    | later versions but |
|                    |                    |                    | all later versions |
|                    |                    |                    | shall be backward  |
|                    |                    |                    | compatible.        |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be "Grain",    |
|                    |                    |                    | "Sugar",           |
|                    |                    |                    | "Extract", "Dry    |
|                    |                    |                    | Extract" or        |
|                    |                    |                    | "Adjunct".<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Extract     |
|                    |                    |                    | refers to liquid   |
|                    |                    |                    | extract.           |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT**         | Yes                | Weight (kg)        | Weight of the      |
|                    |                    |                    | fermentable,       |
|                    |                    |                    | extract or sugar   |
|                    |                    |                    | in Kilograms.      |
+--------------------+--------------------+--------------------+--------------------+
| **YIELD**          | Yes                | Percent            | Percent dry yield  |
|                    |                    |                    | (fine grain) for   |
|                    |                    |                    | the grain, or the  |
|                    |                    |                    | raw yield by       |
|                    |                    |                    | weight if this is  |
|                    |                    |                    | an extract adjunct |
|                    |                    |                    | or sugar.          |
+--------------------+--------------------+--------------------+--------------------+
| **COLOR**          | Yes                | Floating Point     | The color of the   |
|                    |                    |                    | item in Lovibond   |
|                    |                    |                    | Units (SRM for     |
|                    |                    |                    | liquid extracts).  |
+--------------------+--------------------+--------------------+--------------------+
| **ADD\_AFTER\_BOIL | No                 | Boolean            | May be TRUE if     |
| **                 |                    |                    | this item is       |
|                    |                    |                    | normally added     |
|                    |                    |                    | after the          |
|                    |                    |                    | boil.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>The default |
|                    |                    |                    | value is FALSE     |
|                    |                    |                    | since most grains  |
|                    |                    |                    | are added during   |
|                    |                    |                    | the mash or boil.  |
+--------------------+--------------------+--------------------+--------------------+
| **ORIGIN**         | No                 | Text               | Country or place   |
|                    |                    |                    | of origin          |
+--------------------+--------------------+--------------------+--------------------+
| **SUPPLIER**       | No                 | Text               | Supplier of the    |
|                    |                    |                    | grain/extract/suga |
|                    |                    |                    | r                  |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Textual noted      |
|                    |                    |                    | describing this    |
|                    |                    |                    | ingredient and its |
|                    |                    |                    | use.<span          |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be      |
|                    |                    |                    | multiline.         |
+--------------------+--------------------+--------------------+--------------------+
| **COARSE\_FINE\_DI | No                 | Percent            | Percent difference |
| FF**               |                    |                    | between the coarse |
|                    |                    |                    | grain yield and    |
|                    |                    |                    | fine grain         |
|                    |                    |                    | yield.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Only        |
|                    |                    |                    | appropriate for a  |
|                    |                    |                    | "Grain" or         |
|                    |                    |                    | "Adjunct" type,    |
|                    |                    |                    | otherwise this     |
|                    |                    |                    | value is ignored.  |
+--------------------+--------------------+--------------------+--------------------+
| **MOISTURE**       | No                 | Percent            | Percent moisture   |
|                    |                    |                    | in the grain.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Only        |
|                    |                    |                    | appropriate for a  |
|                    |                    |                    | "Grain" or         |
|                    |                    |                    | "Adjunct" type,    |
|                    |                    |                    | otherwise this     |
|                    |                    |                    | value is ignored.  |
+--------------------+--------------------+--------------------+--------------------+
| **DIASTATIC\_POWER | No                 | Floating Point     | The diastatic      |
| **                 |                    |                    | power of the grain |
|                    |                    |                    | as measured in     |
|                    |                    |                    | "Lintner" units.   |
|                    |                    |                    | Only appropriate   |
|                    |                    |                    | for a "Grain" or   |
|                    |                    |                    | "Adjunct" type,    |
|                    |                    |                    | otherwise this     |
|                    |                    |                    | value is ignored.  |
+--------------------+--------------------+--------------------+--------------------+
| **PROTEIN**        | No                 | Percent            | The percent        |
|                    |                    |                    | protein in the     |
|                    |                    |                    | grain.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Only        |
|                    |                    |                    | appropriate for a  |
|                    |                    |                    | "Grain" or         |
|                    |                    |                    | "Adjunct" type,    |
|                    |                    |                    | otherwise this     |
|                    |                    |                    | value is ignored.  |
+--------------------+--------------------+--------------------+--------------------+
| **MAX\_IN\_BATCH** | No                 | Percent            | The recommended    |
|                    |                    |                    | maximum percentage |
|                    |                    |                    | (by weight) this   |
|                    |                    |                    | ingredient should  |
|                    |                    |                    | represent in a     |
|                    |                    |                    | batch of beer.     |
+--------------------+--------------------+--------------------+--------------------+
| **RECOMMEND\_MASH* | No                 | Boolean            | TRUE if it is      |
| *                  |                    |                    | recommended the    |
|                    |                    |                    | grain be mashed,   |
|                    |                    |                    | FALSE if it can be |
|                    |                    |                    | steeped.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>A value of  |
|                    |                    |                    | TRUE is only       |
|                    |                    |                    | appropriate for a  |
|                    |                    |                    | "Grain" or         |
|                    |                    |                    | "Adjunct"          |
|                    |                    |                    | types.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>The default |
|                    |                    |                    | value is           |
|                    |                    |                    | FALSE.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Note that   |
|                    |                    |                    | this does NOT      |
|                    |                    |                    | indicate whether   |
|                    |                    |                    | the grain is       |
|                    |                    |                    | mashed or not – it |
|                    |                    |                    | is only a          |
|                    |                    |                    | recommendation     |
|                    |                    |                    | used in recipe     |
|                    |                    |                    | formulation.       |
+--------------------+--------------------+--------------------+--------------------+
| **IBU\_GAL\_PER\_L | No                 | Floating Point     | For hopped         |
| B**                |                    |                    | extracts only - an |
|                    |                    |                    | estimate of the    |
|                    |                    |                    | number of IBUs per |
|                    |                    |                    | pound of extract   |
|                    |                    |                    | in a gallon of     |
|                    |                    |                    | water.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>To convert  |
|                    |                    |                    | to IBUs we         |
|                    |                    |                    | multiply this      |
|                    |                    |                    | number by the      |
|                    |                    |                    | "AMOUNT" field (in |
|                    |                    |                    | pounds) and divide |
|                    |                    |                    | by the number of   |
|                    |                    |                    | gallons in the     |
|                    |                    |                    | batch.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Based on a  |
|                    |                    |                    | sixty minute       |
|                    |                    |                    | boil.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Only        |
|                    |                    |                    | suitable for use   |
|                    |                    |                    | with an "Extract"  |
|                    |                    |                    | type, otherwise    |
|                    |                    |                    | this value is      |
|                    |                    |                    | ignored.           |
+--------------------+--------------------+--------------------+--------------------+

 

 

**Example Fermentable Record with required fields only:**

 

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Pale 2-row
Malt&lt;/NAME&gt;\]

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;5.0&lt;/AMOUNT&gt;

&lt;TYPE&gt;Grain&lt;/TYPE&gt;

&lt;YIELD&gt;73.4&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;3.0&lt;/COLOR&gt;

&lt;/FERMENTABLE&gt;

 

**Example Hopped Extract:**

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Fustons Hopped
Amber&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.50&lt;/AMOUNT&gt;

&lt;NOTES&gt;Hopped amber extract suitable as a base for english ales.

<span style="mso-spacerun:yes"> </span>&lt;/NOTES&gt;

<span style="mso-spacerun:yes"> </span>&lt;YIELD&gt;78.0&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Extract&lt;/TYPE&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;13&lt;/COLOR&gt;

<span
style="mso-spacerun:yes"> </span>&lt;IBU\_GAL\_PER\_POUND&gt;16.6&lt;/IBU\_GAL\_PER\_POUND&gt;

&lt;/FERMENTABLE&gt;

 

**Sample Crystal Malt Specialty Grain with all applicable fields:**

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Crystal 40
L&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.50&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Grain&lt;/TYPE&gt;

&lt;YIELD&gt;74.0&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;40.0&lt;/COLOR&gt;

<span style="mso-spacerun:yes"> </span>&lt;ORIGIN&gt;United
Kingdom&lt;/ORIGIN&gt;

<span style="mso-spacerun:yes"> </span>&lt;SUPPLIER&gt;Fussybrewer
Malting&lt;/SUPPLIER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Darker crystal malt.

Adds body and improves head retention.

Also called caramel malt.

&lt;/NOTES&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COARSE\_FINE\_DIFF&gt;1.5&lt;/COARSE\_FINE\_DIFF&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MOISTURE&gt;4.0&lt;/MOISTURE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;DIASTATIC\_POWER&gt;0.0&lt;/DISASTATIC\_POWER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PROTEIN&gt;13.2&lt;/PROTEIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAX\_IN\_BATCH&gt;10.0&lt;/MAX\_IN\_BATCH&gt;

<span
style="mso-spacerun:yes"> </span>&lt;RECOMMEND\_MASH&gt;FALSE&lt;/RECOMMEND\_MASH&gt;

&lt;/FERMENTABLE&gt;

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Yeast</span>

The term "yeast" encompasses all yeasts, including dry yeast, liquid
yeast and yeast starters.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **YEAST**          | Yes                | Record             | Starts a yeast     |
|                    |                    |                    | ingredient record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;YEAST&gt;….    |
|                    |                    |                    | &lt;/YEAST&gt;     |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the yeast will  |
|                    |                    |                    | be ignored.        |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the yeast. |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | standard.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should be   |
|                    |                    |                    | “1” for this       |
|                    |                    |                    | version.           |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be “Ale”,      |
|                    |                    |                    | “Lager”, “Wheat”,  |
|                    |                    |                    | “Wine” or          |
|                    |                    |                    | “Champagne”        |
+--------------------+--------------------+--------------------+--------------------+
| **FORM**           | Yes                | List               | May be “Liquid”,   |
|                    |                    |                    | “Dry”, “Slant” or  |
|                    |                    |                    | “Culture”          |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT**         | Yes                | Volume (liters) or | The amount of      |
|                    |                    | Weight (kg)        | yeast, measured in |
|                    |                    |                    | liters.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For a       |
|                    |                    |                    | starter this is    |
|                    |                    |                    | the size of the    |
|                    |                    |                    | starter.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If the flag |
|                    |                    |                    | AMOUNT\_IS\_WEIGHT |
|                    |                    |                    | is set to TRUE     |
|                    |                    |                    | then this          |
|                    |                    |                    | measurement is in  |
|                    |                    |                    | kilograms and not  |
|                    |                    |                    | liters.            |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT\_IS\_WEIG | No                 | Boolean            | TRUE if the amount |
| HT**               |                    |                    | measurement is a   |
|                    |                    |                    | weight measurement |
|                    |                    |                    | and FALSE if the   |
|                    |                    |                    | amount is a volume |
|                    |                    |                    | measurement.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Default     |
|                    |                    |                    | value (if not      |
|                    |                    |                    | present) is        |
|                    |                    |                    | assumed to be      |
|                    |                    |                    | FALSE – therefore  |
|                    |                    |                    | the yeast          |
|                    |                    |                    | measurement is a   |
|                    |                    |                    | liquid amount by   |
|                    |                    |                    | default.           |
+--------------------+--------------------+--------------------+--------------------+
| **LABORATORY**     | No                 | Text               | The name of the    |
|                    |                    |                    | laboratory that    |
|                    |                    |                    | produced the       |
|                    |                    |                    | yeast.             |
+--------------------+--------------------+--------------------+--------------------+
| **PRODUCT\_ID**    | No                 | Text               | The manufacturer’s |
|                    |                    |                    | product ID label   |
|                    |                    |                    | or number that     |
|                    |                    |                    | identifies this    |
|                    |                    |                    | particular strain  |
|                    |                    |                    | of yeast.          |
+--------------------+--------------------+--------------------+--------------------+
| **MIN\_TEMPERATURE | No                 | Temperature (C)    | The minimum        |
| **                 |                    |                    | recommended        |
|                    |                    |                    | temperature for    |
|                    |                    |                    | fermenting this    |
|                    |                    |                    | yeast strain in    |
|                    |                    |                    | degrees Celsius.   |
+--------------------+--------------------+--------------------+--------------------+
| **MAX\_TEMPERATURE | No                 | Temperature (C)    | The maximum        |
| **                 |                    |                    | recommended        |
|                    |                    |                    | temperature for    |
|                    |                    |                    | fermenting this    |
|                    |                    |                    | yeast strain in    |
|                    |                    |                    | Celsius.           |
+--------------------+--------------------+--------------------+--------------------+
| **FLOCCULATION**   | No                 | List               | May be “Low”,      |
|                    |                    |                    | “Medium”, “High”   |
|                    |                    |                    | or “Very High”     |
+--------------------+--------------------+--------------------+--------------------+
| **ATTENUATION**    | No                 | Percent            | Average            |
|                    |                    |                    | attenuation for    |
|                    |                    |                    | this yeast strain. |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Notes on this      |
|                    |                    |                    | yeast strain.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be a    |
|                    |                    |                    | multiline entry.   |
+--------------------+--------------------+--------------------+--------------------+
| **BEST\_FOR**      | No                 | Text               | Styles or types of |
|                    |                    |                    | beer this yeast    |
|                    |                    |                    | strain is best     |
|                    |                    |                    | suited for.        |
+--------------------+--------------------+--------------------+--------------------+
| **TIMES\_CULTURED* | No                 | Integer            | Number of times    |
| *                  |                    |                    | this yeast has     |
|                    |                    |                    | been reused as a   |
|                    |                    |                    | harvested          |
|                    |                    |                    | culture.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>This number |
|                    |                    |                    | should be zero if  |
|                    |                    |                    | this is a product  |
|                    |                    |                    | directly from the  |
|                    |                    |                    | manufacturer.      |
+--------------------+--------------------+--------------------+--------------------+
| **MAX\_REUSE**     | No                 | Integer            | Recommended of     |
|                    |                    |                    | times this yeast   |
|                    |                    |                    | can be reused      |
|                    |                    |                    | (recultured from a |
|                    |                    |                    | previous batch)    |
+--------------------+--------------------+--------------------+--------------------+
| **ADD\_TO\_SECONDA | No                 | Boolean            | Flag denoting that |
| RY**               |                    |                    | this yeast was     |
|                    |                    |                    | added for a        |
|                    |                    |                    | secondary (or      |
|                    |                    |                    | later)             |
|                    |                    |                    | fermentation as    |
|                    |                    |                    | opposed to the     |
|                    |                    |                    | primary            |
|                    |                    |                    | fermentation.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Useful if   |
|                    |                    |                    | one uses two or    |
|                    |                    |                    | more yeast strains |
|                    |                    |                    | for a single brew  |
|                    |                    |                    | (eg: Lambic).<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Default     |
|                    |                    |                    | value is FALSE.    |
+--------------------+--------------------+--------------------+--------------------+

 

 

**Example: Yeast with required fields only**

&lt;YEAST&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Ole English Ale
Yeast&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Ale&lt;/TYPE&gt;

<span style="mso-spacerun:yes"> </span>&lt;FORM&gt;Liquid&lt;/FORM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.100&lt;/AMOUNT&gt;

&lt;/YEAST&gt;

 

**Example: Yeast with more popular fields**

&lt;YEAST&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;German
Ale&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;TYPE&gt;Ale&lt;/TYPE&gt;</span>

<span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;FORM&gt;Liquid&lt;/FORM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.250&lt;/AMOUNT&gt;

&lt;LABORATORY&gt;Wyeast Labs&lt;/LABORATORY&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PRODUCT\_ID&gt;1007&lt;/PRODUCT\_ID&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MIN\_TEMPERATURE&gt;12.8&lt;/MIN\_TEMPERATURE&gt;

&lt;MAX\_TEMPERATURE&gt;20.0&lt;/MAX\_TEMPERATURE&gt;

&lt;ATTENUATION&gt;75.0&lt;/ATTENUATION&gt;

&lt;NOTES&gt;Crisp dry flavor with a hint of mild flavor.

<span style="mso-spacerun:yes">  </span>Great for many continental ales.

<span style="mso-spacerun:yes"> </span>&lt;/NOTES&gt;

<span style="mso-spacerun:yes"> </span>&lt;BEST\_FOR&gt;German Ales,
Alts, Kolsch, Dry Stouts &lt;/BEST\_FOR&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FLOCCULATION&gt;Low&lt;/FLOCCULATION&gt;

&lt;/YEAST&gt;

 

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Misc</span>

The term "misc" encompasses all non-fermentable miscellaneous
ingredients that are not hops or yeast and do not significantly change
the gravity of the beer.<span style="mso-spacerun:yes">  </span>For
example: spices, clarifying agents, water treatments, etc…

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **MISC**           | Yes                | Record             | Starts a misc      |
|                    |                    |                    | ingredient record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;MISC&gt;….     |
|                    |                    |                    | &lt;/MISC&gt;      |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the misc will   |
|                    |                    |                    | be ignored.        |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the misc   |
|                    |                    |                    | item.              |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version number of  |
|                    |                    |                    | this element.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should be   |
|                    |                    |                    | “1” for this       |
|                    |                    |                    | version.           |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be “Spice”,    |
|                    |                    |                    | “Fining”, “Water   |
|                    |                    |                    | Agent”, “Herb”,    |
|                    |                    |                    | “Flavor” or        |
|                    |                    |                    | “Other”            |
+--------------------+--------------------+--------------------+--------------------+
| **USE**            | Yes                | List               | May be “Boil”,     |
|                    |                    |                    | “Mash”, “Primary”, |
|                    |                    |                    | “Secondary”,       |
|                    |                    |                    | “Bottling”         |
+--------------------+--------------------+--------------------+--------------------+
| **TIME**           | Yes                | Time (min)         | Amount of time the |
|                    |                    |                    | misc was boiled,   |
|                    |                    |                    | steeped, mashed,   |
|                    |                    |                    | etc in minutes.    |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT**         | Yes                | Volume (l) or      | Amount of item     |
|                    |                    | Weight (kg)        | used.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>The default |
|                    |                    |                    | measurements are   |
|                    |                    |                    | by weight, but     |
|                    |                    |                    | this may be the    |
|                    |                    |                    | measurement in     |
|                    |                    |                    | volume units if    |
|                    |                    |                    | AMOUNT\_IS\_WEIGHT |
|                    |                    |                    | is set to TRUE for |
|                    |                    |                    | this record.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If a liquid |
|                    |                    |                    | it is in liters,   |
|                    |                    |                    | if a solid the     |
|                    |                    |                    | weight is measured |
|                    |                    |                    | in kilograms.      |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT\_IS\_WEIG | No                 | Boolean            | TRUE if the amount |
| HT**               |                    |                    | measurement is a   |
|                    |                    |                    | weight measurement |
|                    |                    |                    | and FALSE if the   |
|                    |                    |                    | amount is a volume |
|                    |                    |                    | measurement.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Default     |
|                    |                    |                    | value (if not      |
|                    |                    |                    | present) is        |
|                    |                    |                    | assumed to be      |
|                    |                    |                    | FALSE.             |
+--------------------+--------------------+--------------------+--------------------+
| **USE\_FOR**       | No                 | Text               | Short description  |
|                    |                    |                    | of what the        |
|                    |                    |                    | ingredient is used |
|                    |                    |                    | for in text        |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Detailed notes on  |
|                    |                    |                    | the item including |
|                    |                    |                    | usage.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be      |
|                    |                    |                    | multiline.         |
+--------------------+--------------------+--------------------+--------------------+

 

**Example: Irish Moss misc with minimal fields**

&lt;MISC&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Irish
Moss&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Fining&lt;/TYPE&gt;

<span style="mso-spacerun:yes"> </span>&lt;USE&gt;Boil&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;TIME&gt;15.0&lt;/TIME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.010&lt;/AMOUNT&gt;

&lt;/MISC&gt;

 

**Example: Coriander Spice with a typical set of fields**

&lt;MISC&gt;

<span
style="mso-spacerun:yes"> </span>&lt;NAME&gt;Coriander&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Spice&lt;/TYPE&gt;

&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;USE&gt;Boil&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;TIME&gt;5.0&lt;/TIME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.025&lt;/AMOUNT&gt;

&lt;USE\_FOR&gt;Belgian Wit Spice&lt;/USE\_FOR&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Used in Belgian Wit,
Whites, and Holiday ales.<span style="mso-spacerun:yes">  </span>Very
good when used in light wheat ales.<span style="mso-spacerun:yes"> 
</span>Often used with Bitter Orange Peel. Crack open seeds and add at
the end of the boil to extract aroma and flavor.

&lt;/NOTES&gt;

&lt;/MISC&gt;

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Water</span>

The term "water" encompasses water profiles.<span
style="mso-spacerun:yes">  </span>Though not strictly required for
recipes, the water record allows supporting programs to record the water
profile used for brewing a particular batch.<span
style="mso-spacerun:yes">  </span>

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **WATER**          | Yes                | Record             | Starts a WATER     |
|                    |                    |                    | ingredient record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;WATER&gt;….    |
|                    |                    |                    | &lt;/WATER&gt;     |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the water will  |
|                    |                    |                    | be ignored.        |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the water  |
|                    |                    |                    | profile – usually  |
|                    |                    |                    | the city and       |
|                    |                    |                    | country of the     |
|                    |                    |                    | water profile.     |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | water record.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **AMOUNT**         | Yes                | Volume (liters)    | Volume of water to |
|                    |                    |                    | use in a recipe in |
|                    |                    |                    | liters.            |
+--------------------+--------------------+--------------------+--------------------+
| **CALCIUM**        | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | calcium (Ca) in    |
|                    |                    |                    | parts per million. |
+--------------------+--------------------+--------------------+--------------------+
| **BICARBONATE**    | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | bicarbonate (HCO3) |
|                    |                    |                    | in parts per       |
|                    |                    |                    | million.           |
+--------------------+--------------------+--------------------+--------------------+
| **SULFATE**        | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | Sulfate (SO4) in   |
|                    |                    |                    | parts per million. |
+--------------------+--------------------+--------------------+--------------------+
| **CHLORIDE**       | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | Chloride (Cl) in   |
|                    |                    |                    | parts per million. |
+--------------------+--------------------+--------------------+--------------------+
| **SODIUM**         | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | Sodium (Na) in     |
|                    |                    |                    | parts per million. |
+--------------------+--------------------+--------------------+--------------------+
| **MAGNESIUM**      | Yes                | Floating Point     | The amount of      |
|                    |                    |                    | Magnesium (Mg) in  |
|                    |                    |                    | parts per million. |
+--------------------+--------------------+--------------------+--------------------+
| **PH**             | No                 | Floating Point     | The PH of the      |
|                    |                    |                    | water.             |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Notes about the    |
|                    |                    |                    | water              |
|                    |                    |                    | profile.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be      |
|                    |                    |                    | multiline.         |
+--------------------+--------------------+--------------------+--------------------+

 

**Example: A Sample Water Profile**

&lt;WATER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Burton on Trent,
UK&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;20.0&lt;/AMOUNT&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CALCIUM&gt;295.0&lt;/CALCIUM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAGNESIUM&gt;45.0&lt;/MAGNESIUM&gt;

<span style="mso-spacerun:yes"> </span>&lt;SODIUM&gt;55.0&lt;/SODIUM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;SULFATE&gt;725.0&lt;/SULFATE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CHLORIDE&gt;25.0&lt;/CHLORIDE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;BICARBONATE&gt;300.0&lt;/BICARBONATE&gt;

<span style="mso-spacerun:yes"> </span>&lt;PH&gt;8.0&lt;/PH&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;

Use for distinctive pale ales strongly hopped.<span
style="mso-spacerun:yes">  </span>Very hard water accentuates the hops
flavor. Example: Bass Ale

&lt;/NOTES&gt;

&lt;/WATER&gt;

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Equipment</span>

 

Though an equipment record is optional, when used it in a recipe or on
its own it provides details needed to calculate total water usage as
well as water needed for each step.<span style="mso-spacerun:yes"> 
</span>It also contains information about the thermal parameters of the
mash tun and large batch hop utilization factors.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **EQUIPMENT**      | Yes                | Record             | Starts a EQUIPMENT |
|                    |                    |                    | record -- any of   |
|                    |                    |                    | the below tags may |
|                    |                    |                    | be included in any |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;EQUIPMENT&gt;… |
|                    |                    |                    | .                  |
|                    |                    |                    | &lt;/EQUIPMENT&gt; |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the equipment   |
|                    |                    |                    | will be ignored.   |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the        |
|                    |                    |                    | equipment profile  |
|                    |                    |                    | – usually a text   |
|                    |                    |                    | description of the |
|                    |                    |                    | brewing setup.     |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | equipment          |
|                    |                    |                    | record.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **BOIL\_SIZE**     | Yes                | Volume (liters)    | The pre-boil       |
|                    |                    |                    | volume used in     |
|                    |                    |                    | this particular    |
|                    |                    |                    | instance for this  |
|                    |                    |                    | equipment          |
|                    |                    |                    | setup.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Note that   |
|                    |                    |                    | this may be a      |
|                    |                    |                    | calculated value   |
|                    |                    |                    | depending on the   |
|                    |                    |                    | CALC\_BOIL\_VOLUME |
|                    |                    |                    | parameter.         |
+--------------------+--------------------+--------------------+--------------------+
| **BATCH\_SIZE**    | Yes                | Volume (liters)    | The target volume  |
|                    |                    |                    | of the batch at    |
|                    |                    |                    | the start of       |
|                    |                    |                    | fermentation.      |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_VOLUME**    | No                 | Volume (liters)    | Volume of the mash |
|                    |                    |                    | tun in             |
|                    |                    |                    | liters.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>This        |
|                    |                    |                    | parameter can be   |
|                    |                    |                    | used to calculate  |
|                    |                    |                    | if a particular    |
|                    |                    |                    | mash and grain     |
|                    |                    |                    | profile will fit   |
|                    |                    |                    | in the mash        |
|                    |                    |                    | tun.<span          |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>It may also |
|                    |                    |                    | be used for        |
|                    |                    |                    | thermal            |
|                    |                    |                    | calculations in    |
|                    |                    |                    | the case of a      |
|                    |                    |                    | partially full     |
|                    |                    |                    | mash tun.          |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_WEIGHT**    | No                 | Weight (kg)        | Weight of the mash |
|                    |                    |                    | tun in             |
|                    |                    |                    | kilograms.<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Used        |
|                    |                    |                    | primarily to       |
|                    |                    |                    | calculate the      |
|                    |                    |                    | thermal parameters |
|                    |                    |                    | of the mash tun –  |
|                    |                    |                    | in conjunction     |
|                    |                    |                    | with the volume    |
|                    |                    |                    | and specific heat. |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_SPECIFIC\_H | No                 | Cal/gram-deg C     | The specific heat  |
| EAT**              |                    |                    | of the mash tun    |
|                    |                    |                    | which is usually a |
|                    |                    |                    | function of the    |
|                    |                    |                    | material it is     |
|                    |                    |                    | made of.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Typical     |
|                    |                    |                    | ranges are         |
|                    |                    |                    | 0.1-0.25 for metal |
|                    |                    |                    | and 0.2-0.5 for    |
|                    |                    |                    | plastic materials. |
+--------------------+--------------------+--------------------+--------------------+
| **TOP\_UP\_WATER** | No                 | Volume (liters)    | The amount of top  |
|                    |                    |                    | up water normally  |
|                    |                    |                    | added just prior   |
|                    |                    |                    | to starting        |
|                    |                    |                    | fermentation.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Usually     |
|                    |                    |                    | used for extract   |
|                    |                    |                    | brewing.           |
+--------------------+--------------------+--------------------+--------------------+
| **TRUB\_CHILLER\_L | No                 | Volume (liters)    | The amount of wort |
| OSS**              |                    |                    | normally lost      |
|                    |                    |                    | during transition  |
|                    |                    |                    | from the boiler to |
|                    |                    |                    | the fermentation   |
|                    |                    |                    | vessel.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Includes    |
|                    |                    |                    | both unusable wort |
|                    |                    |                    | due to trub and    |
|                    |                    |                    | wort lost to the   |
|                    |                    |                    | chiller and        |
|                    |                    |                    | transfer systems.  |
+--------------------+--------------------+--------------------+--------------------+
| **EVAP\_RATE**     | No                 | Percent per hour   | The percentage of  |
|                    |                    |                    | wort lost to       |
|                    |                    |                    | evaporation per    |
|                    |                    |                    | hour of the boil.  |
+--------------------+--------------------+--------------------+--------------------+
| **BOIL\_TIME**     | No                 | Normal boil time   | The normal amount  |
|                    |                    |                    | of time one boils  |
|                    |                    |                    | for this equipment |
|                    |                    |                    | setup. <span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes"> </span>Thi |
|                    |                    |                    | s                  |
|                    |                    |                    | can be used with   |
|                    |                    |                    | the evaporation    |
|                    |                    |                    | rate to calculate  |
|                    |                    |                    | the evaporation    |
|                    |                    |                    | loss.              |
+--------------------+--------------------+--------------------+--------------------+
| **CALC\_BOIL\_VOLU | No                 | Boolean            | Flag denoting that |
| ME**               |                    |                    | the program should |
|                    |                    |                    | calculate the boil |
|                    |                    |                    | size.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Flag may be |
|                    |                    |                    | TRUE or            |
|                    |                    |                    | FALSE.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If TRUE,    |
|                    |                    |                    | then BOIL\_SIZE =  |
|                    |                    |                    | (BATCH\_SIZE –     |
|                    |                    |                    | TOP\_UP\_WATER –   |
|                    |                    |                    | TRUB\_CHILLER\_LOS |
|                    |                    |                    | S)                 |
|                    |                    |                    | \* (1+BOIL\_TIME   |
|                    |                    |                    | \* EVAP\_RATE      |
|                    |                    |                    | )<span             |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If set then |
|                    |                    |                    | the boil size      |
|                    |                    |                    | should match this  |
|                    |                    |                    | value.             |
+--------------------+--------------------+--------------------+--------------------+
| **LAUTER\_DEADSPAC | No                 | Volume (liters)    | Amount lost to the |
| E**                |                    |                    | lauter tun and     |
|                    |                    |                    | equipment          |
|                    |                    |                    | associated with    |
|                    |                    |                    | the lautering      |
|                    |                    |                    | process.           |
+--------------------+--------------------+--------------------+--------------------+
| **TOP\_UP\_KETTLE* | No                 | Volume (liters)    | Amount normally    |
| *                  |                    |                    | added to the boil  |
|                    |                    |                    | kettle before the  |
|                    |                    |                    | boil.              |
+--------------------+--------------------+--------------------+--------------------+
| **HOP\_UTILIZATION | No                 | Percent            | Large batch hop    |
| **                 |                    |                    | utilization.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>This value  |
|                    |                    |                    | should be 100% for |
|                    |                    |                    | batches less than  |
|                    |                    |                    | 20 gallons, but    |
|                    |                    |                    | may be higher      |
|                    |                    |                    | (200% or more) for |
|                    |                    |                    | very large batch   |
|                    |                    |                    | equipment.         |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Notes associated   |
|                    |                    |                    | with the           |
|                    |                    |                    | equipment.<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>May be a    |
|                    |                    |                    | multiline entry.   |
+--------------------+--------------------+--------------------+--------------------+

 

Example:

 

&lt;EQUIPMENT&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;8 Gal pot with 5 gal
Igloo Cooler&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;TUN\_VOLUME&gt;18.93&lt;/MASH\_TUN\_VOLUME&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;TUN\_WEIGHT&gt;2.0&lt;/MASH\_TUN\_WEIGHT&gt;

<span
style="mso-spacerun:yes"> </span>&lt;TUN\_SPECIFIC\_HEAT&gt;0.3&lt;/TUN\_SPECIFIC\_HEAT&gt;

<span
style="mso-spacerun:yes"> </span>&lt;BATCH\_SIZE&gt;18.93&lt;/BATCH\_SIZE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;BOIL\_SIZE&gt;22.71&lt;/BOIL\_SIZE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;TOP\_UP\_WATER&gt;0.0&lt;/TOP\_UP\_WATER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;TRUB\_CHILLER\_LOSS&gt;0.95&lt;/TRUB\_CHILLER\_LOSS&gt;

<span
style="mso-spacerun:yes"> </span>&lt;EVAP\_RATE&gt;9.0&lt;/EVAP\_RATE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;BOIL\_TIME&gt;60.0&lt;/BOIL\_TIME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CALC\_BOIL\_VOLUME&gt;TRUE&lt;/CALC\_BOIL\_VOLUME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;LAUTER\_DEADSPACE&gt;0.95&lt;/LAUTER\_DEADSPACE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;TOP\_UP\_KETTLE&gt;0.0&lt;/TOP\_UP\_KETTLE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;HOP\_UTILIZATION&gt;100.0&lt;/HOP\_UTILIZATION&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Popular all grain
setup.<span style="mso-spacerun:yes">  </span>5 Gallon Gott or Igloo
cooler as mash tun with false bottom, and 7-9 gallon brewpot capable of
boiling at least 6 gallons of wort.<span style="mso-spacerun:yes"> 
</span>Primarily used for single infusion mashes.&lt;/NOTES&gt;

&lt;/EQUIPMENT&gt;

 

<span style="color:#3366FF"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Style</span>

The term "style" encompasses beer styles.<span
style="mso-spacerun:yes">  </span>The beer style may be from the BJCP
style guide, Australian, UK or local style guides.<span
style="mso-spacerun:yes">  </span>Generally a recipe is designed to one
style.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **STYLE**          | Yes                | Record             | Starts a STYLE     |
|                    |                    |                    | record -- any of   |
|                    |                    |                    | the below tags may |
|                    |                    |                    | be included in any |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;STYLE&gt;….    |
|                    |                    |                    | &lt;/STYLE&gt;     |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the style will  |
|                    |                    |                    | be ignored.        |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the style  |
|                    |                    |                    | profile – usually  |
|                    |                    |                    | this is the        |
|                    |                    |                    | specific name of   |
|                    |                    |                    | the style – for    |
|                    |                    |                    | example “Scottish  |
|                    |                    |                    | Wee Heavy Ale” and |
|                    |                    |                    | not the Category   |
|                    |                    |                    | which in this case |
|                    |                    |                    | might be “Scottish |
|                    |                    |                    | Ale”               |
+--------------------+--------------------+--------------------+--------------------+
| **CATEGORY**       | Yes                | Text               | Category that this |
|                    |                    |                    | style belongs to – |
|                    |                    |                    | usually associated |
|                    |                    |                    | with a group of    |
|                    |                    |                    | styles such as     |
|                    |                    |                    | “English Ales” or  |
|                    |                    |                    | “Amercian Lagers”. |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | style record.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **CATEGORY\_NUMBER | Yes                | Text               | Number or          |
| **                 |                    |                    | identifier         |
|                    |                    |                    | associated with    |
|                    |                    |                    | this style         |
|                    |                    |                    | category.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | in the BJCP style  |
|                    |                    |                    | guide, the         |
|                    |                    |                    | “American Lager”   |
|                    |                    |                    | category has a     |
|                    |                    |                    | category number of |
|                    |                    |                    | “1”.               |
+--------------------+--------------------+--------------------+--------------------+
| **STYLE\_LETTER**  | Yes                | Text               | The specific style |
|                    |                    |                    | number or          |
|                    |                    |                    | subcategory letter |
|                    |                    |                    | associated with    |
|                    |                    |                    | this particular    |
|                    |                    |                    | style.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | in the BJCP style  |
|                    |                    |                    | guide, an American |
|                    |                    |                    | Standard Lager     |
|                    |                    |                    | would be style     |
|                    |                    |                    | letter “A” under   |
|                    |                    |                    | the main           |
|                    |                    |                    | category.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Letters     |
|                    |                    |                    | should be upper    |
|                    |                    |                    | case.              |
+--------------------+--------------------+--------------------+--------------------+
| **STYLE\_GUIDE**   | Yes                | Text               | The name of the    |
|                    |                    |                    | style guide that   |
|                    |                    |                    | this particular    |
|                    |                    |                    | style or category  |
|                    |                    |                    | belongs to.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “BJCP” might       |
|                    |                    |                    | denote the BJCP    |
|                    |                    |                    | style guide, and   |
|                    |                    |                    | “AHA” would be     |
|                    |                    |                    | used for the AHA   |
|                    |                    |                    | style guide.       |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be “Lager”,    |
|                    |                    |                    | “Ale”, “Mead”,     |
|                    |                    |                    | “Wheat”, “Mixed”   |
|                    |                    |                    | or “Cider”.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Defines the |
|                    |                    |                    | type of beverage   |
|                    |                    |                    | associated with    |
|                    |                    |                    | this category.     |
+--------------------+--------------------+--------------------+--------------------+
| **OG\_MIN**        | Yes                | Specific Gravity   | The minimum        |
|                    |                    |                    | specific gravity   |
|                    |                    |                    | as measured        |
|                    |                    |                    | relative to        |
|                    |                    |                    | water.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.040” might be a |
|                    |                    |                    | reasonable minimum |
|                    |                    |                    | for a Pale Ale.    |
+--------------------+--------------------+--------------------+--------------------+
| **OG\_MAX**        | Yes                | Specific Gravity   | The maximum        |
|                    |                    |                    | specific gravity   |
|                    |                    |                    | as measured        |
|                    |                    |                    | relative to water. |
+--------------------+--------------------+--------------------+--------------------+
| **FG\_MIN**        | Yes                | Specific Gravity   | The minimum final  |
|                    |                    |                    | gravity as         |
|                    |                    |                    | measured relative  |
|                    |                    |                    | to water.          |
+--------------------+--------------------+--------------------+--------------------+
| **FG\_MAX**        | Yes                | Specific Gravity   | The maximum final  |
|                    |                    |                    | gravity as         |
|                    |                    |                    | measured relative  |
|                    |                    |                    | to water.          |
+--------------------+--------------------+--------------------+--------------------+
| **IBU\_MIN**       | Yes                | IBUs               | The recommended    |
|                    |                    |                    | minimum bitterness |
|                    |                    |                    | for this style as  |
|                    |                    |                    | measured in        |
|                    |                    |                    | International      |
|                    |                    |                    | Bitterness Units   |
|                    |                    |                    | (IBUs)             |
+--------------------+--------------------+--------------------+--------------------+
| **IBU\_MAX**       | Yes                | IBUs               | The recommended    |
|                    |                    |                    | maximum bitterness |
|                    |                    |                    | for this style as  |
|                    |                    |                    | measured in        |
|                    |                    |                    | International      |
|                    |                    |                    | Bitterness Units   |
|                    |                    |                    | (IBUs)             |
+--------------------+--------------------+--------------------+--------------------+
| **COLOR\_MIN**     | Yes                | SRM Color          | The minimum        |
|                    |                    |                    | recommended color  |
|                    |                    |                    | in SRM             |
+--------------------+--------------------+--------------------+--------------------+
| **COLOR\_MAX**     | Yes                | SRM Color          | The maximum        |
|                    |                    |                    | recommended color  |
|                    |                    |                    | in SRM.            |
+--------------------+--------------------+--------------------+--------------------+
| **CARB\_MIN**      | No                 | Volumes of CO2     | Minimum            |
|                    |                    |                    | recommended        |
|                    |                    |                    | carbonation for    |
|                    |                    |                    | this style in      |
|                    |                    |                    | volumes of CO2     |
+--------------------+--------------------+--------------------+--------------------+
| **CARB\_MAX**      | No                 | Volumes of CO2     | The maximum        |
|                    |                    |                    | recommended        |
|                    |                    |                    | carbonation for    |
|                    |                    |                    | this style in      |
|                    |                    |                    | volumes of CO2     |
+--------------------+--------------------+--------------------+--------------------+
| **ABV\_MIN**       | No                 | Percent            | The minimum        |
|                    |                    |                    | recommended        |
|                    |                    |                    | alcohol by volume  |
|                    |                    |                    | as a percentage.   |
+--------------------+--------------------+--------------------+--------------------+
| **ABV\_MAX**       | No                 | Percent            | The maximum        |
|                    |                    |                    | recommended        |
|                    |                    |                    | alcohol by volume  |
|                    |                    |                    | as a percentage.   |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Description of the |
|                    |                    |                    | style, history     |
+--------------------+--------------------+--------------------+--------------------+
| **PROFILE**        | No                 | Text               | Flavor and aroma   |
|                    |                    |                    | profile for this   |
|                    |                    |                    | style              |
+--------------------+--------------------+--------------------+--------------------+
| **INGREDIENTS**    | No                 | Text               | Suggested          |
|                    |                    |                    | ingredients for    |
|                    |                    |                    | this style         |
+--------------------+--------------------+--------------------+--------------------+
| **EXAMPLES**       | No                 | Text               | Example beers of   |
|                    |                    |                    | this style.        |
+--------------------+--------------------+--------------------+--------------------+

 

**Example: Bohemian Pilsner**

&lt;STYLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Bohemian
Pilsner&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;CATEGORY&gt;European Pale
Ale&lt;/CATEGORY&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CATEGORY\_NUMBER&gt;2&lt;/CATEGORY\_NUMBER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STYLE\_LETTER&gt;A&lt;/STYLE\_LETTER&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;STYLE\_GUIDE&gt;BJCP&lt;/STYLE\_GUIDE&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;TYPE&gt;Lager&lt;/TYPE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MIN&gt;1.044&lt;/OG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MAX&gt;1.056&lt;/OG\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MIN&gt;1.013&lt;/FG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MAX&gt;1.017&lt;/FG\_MAX&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;IBU\_MIN&gt;35.0&lt;/IBU\_MIN&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;IBU\_MAX&gt;45.0&lt;/IBU\_MAX&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;COLOR\_MIN&gt;3.0&lt;/COLOR\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COLOR\_MAX&gt;5.0&lt;/COLOR\_MAX&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Famous beer of
Pilsen, Czech republic.<span style="mso-spacerun:yes">  </span>Light to
medium body with some sweetness.<span style="mso-spacerun:yes"> 
</span>Saaz hop flavor and aroma with no lingering bitterness.

&lt;/NOTES&gt;

&lt;/STYLE&gt;

 

**Example: Dry Irish Stout with all fields**

&lt;STYLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Dry
Stout&lt;/NAME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CATEGORY&gt;Stout&lt;/CATEGORY&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CATEGORY\_NUMBER&gt;16&lt;/CATEGORY\_NUMBER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STYLE\_LETTER&gt;A&lt;/STYLE\_LETTER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STYLE\_GUIDE&gt;BJCP&lt;/STYLE\_GUIDE&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Ale&lt;/TYPE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MIN&gt;1.035&lt;/OG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MAX&gt;1.050&lt;/OG\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MIN&gt;1.007&lt;/FG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MAX&gt;1.011&lt;/FG\_MAX&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;IBU\_MIN&gt;30.0&lt;/IBU\_MIN&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;IBU\_MAX&gt;50.0&lt;/IBU\_MAX&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;COLOR\_MIN&gt;35.0&lt;/COLOR\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COLOR\_MAX&gt;200.0&lt;/COLOR\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;ABV\_MIN&gt;3.2&lt;/ABV\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;ABV\_MAX&gt;5.5&lt;/ABV\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CARB\_MIN&gt;1.6&lt;/CARB\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CARB\_MAX&gt;2.1&lt;/CARB\_MAX&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Famous Irish
Stout.<span style="mso-spacerun:yes">  </span>Dry, roasted, almost
coffee like flavor.<span style="mso-spacerun:yes">  </span>Often soured
with pasteurized sour beer.<span style="mso-spacerun:yes">  </span>

&lt;/NOTES&gt;

&lt;PROFILE&gt;Full body perception due to flaked barley, though
starting gravity may be low.<span style="mso-spacerun:yes">  </span>Dry
roasted flavor.

&lt;/PROFILE&gt;

&lt;INGREDIENTS&gt;Made with black barley and flaked barley,<span
style="mso-spacerun:yes">  </span>Hard water.<span
style="mso-spacerun:yes">  </span>Irish Ale Yeast.

&lt;/INGREDIENTS&gt;

&lt;EXAMPLES&gt;Guinness&lt;/EXAMPLES&gt;

&lt;/STYLE&gt;

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Mash Step</span>

A mash step is an internal record used within a mash profile to denote a
separate step in a multi-step mash.<span style="mso-spacerun:yes"> 
</span>A mash step is not intended for use outside of a mash profile.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **MASH\_STEP**     | Yes                | Record             | Starts a           |
|                    |                    |                    | MASH\_STEP record  |
|                    |                    |                    | -- any of the      |
|                    |                    |                    | below tags may be  |
|                    |                    |                    | included in any    |
|                    |                    |                    | order within the   |
|                    |                    |                    | &lt;MASH\_STEP&gt; |
|                    |                    |                    | ….                 |
|                    |                    |                    | &lt;/MASH\_STEP&gt |
|                    |                    |                    | ;                  |
|                    |                    |                    | record tags.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Any         |
|                    |                    |                    | non-standard tags  |
|                    |                    |                    | in the mash step   |
|                    |                    |                    | will be ignored.   |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the mash   |
|                    |                    |                    | step – usually     |
|                    |                    |                    | descriptive text   |
|                    |                    |                    | such as “Dough In” |
|                    |                    |                    | or “Conversion”    |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | mash step          |
|                    |                    |                    | record.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be “Infusion”, |
|                    |                    |                    | “Temperature” or   |
|                    |                    |                    | “Decoction”        |
|                    |                    |                    | depending on the   |
|                    |                    |                    | type of step.<span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Infusion    |
|                    |                    |                    | denotes adding hot |
|                    |                    |                    | water, Temperature |
|                    |                    |                    | denotes heating    |
|                    |                    |                    | with an outside    |
|                    |                    |                    | heat source, and   |
|                    |                    |                    | decoction denotes  |
|                    |                    |                    | drawing off some   |
|                    |                    |                    | mash for boiling.  |
+--------------------+--------------------+--------------------+--------------------+
| **INFUSE\_AMOUNT** | Conditional        | Volume (liters)    | The volume of      |
|                    |                    |                    | water in liters to |
|                    |                    |                    | infuse in this     |
|                    |                    |                    | step.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Required    |
|                    |                    |                    | only for infusion  |
|                    |                    |                    | steps, though one  |
|                    |                    |                    | may also add water |
|                    |                    |                    | for temperature    |
|                    |                    |                    | mash steps.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>One should  |
|                    |                    |                    | not have an        |
|                    |                    |                    | infusion amount    |
|                    |                    |                    | for decoction      |
|                    |                    |                    | steps.             |
+--------------------+--------------------+--------------------+--------------------+
| **STEP\_TEMP**     | Yes                | Temperature (C)    | The target         |
|                    |                    |                    | temperature for    |
|                    |                    |                    | this step in       |
|                    |                    |                    | degrees Celsius.   |
+--------------------+--------------------+--------------------+--------------------+
| **STEP\_TIME**     | Yes                | Time in Minutes    | The number of      |
|                    |                    |                    | minutes to spend   |
|                    |                    |                    | at this step –     |
|                    |                    |                    | i.e. the amount of |
|                    |                    |                    | time we are to     |
|                    |                    |                    | hold this          |
|                    |                    |                    | particular step    |
|                    |                    |                    | temperature.       |
+--------------------+--------------------+--------------------+--------------------+
| **RAMP\_TIME**     | No                 | Minutes            | Time in minutes to |
|                    |                    |                    | achieve the        |
|                    |                    |                    | desired step       |
|                    |                    |                    | temperature –      |
|                    |                    |                    | useful             |
|                    |                    |                    | particularly for   |
|                    |                    |                    | temperature mashes |
|                    |                    |                    | where it may take  |
|                    |                    |                    | some time to       |
|                    |                    |                    | achieve the step   |
|                    |                    |                    | temperature.       |
+--------------------+--------------------+--------------------+--------------------+
| **END\_TEMP**      | No                 | Temperature<span   | the temperature    |
|                    |                    | style="mso-spaceru | you can expect the |
|                    |                    | n:yes">            | mash to fall to    |
|                    |                    | </span>(Celsius)   | after a long mash  |
|                    |                    |                    | step.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Measured in |
|                    |                    |                    | degrees Celsius.   |
+--------------------+--------------------+--------------------+--------------------+

 

**Example: Infusion Step add 5 liters – 68 C for 70 minutes**

&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Conversion
step&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Infusion&lt;/TYPE&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span></span>&lt;STEP\_TEMP&gt;68.0&lt;/STEP\_TEMP&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STEP\_TIME&gt;70.0&lt;/STEP\_TIME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;INFUSE\_AMOUNT&gt;5.0&lt;/INFUSE\_AMOUNT&gt;

&lt;/MASH\_STEP&gt;

 

**Example: Decoction Step – 68C <span
style="mso-spacerun:yes"> </span>for 90 minutes**

&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Conversion
Decoction&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span
style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Decoction&lt;/TYPE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STEP\_TEMP&gt;68.0&lt;/STEP\_TEMP&gt;

&lt;STEP\_TIME&gt;90.0&lt;/STEP\_TIME&gt;

&lt;/MASH\_STEP&gt;

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Mash Profile</span>

A mash profile is a record used either within a recipe or outside the
recipe to precisely specify the mash method used.<span
style="mso-spacerun:yes">  </span>The record consists of some
informational items followed by a &lt;MASH\_STEPS&gt; record that
contains the actual mash steps.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **MASH**           | Yes                | Record             | Starts a MASH      |
|                    |                    |                    | profile            |
|                    |                    |                    | record.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>All items   |
|                    |                    |                    | below should       |
|                    |                    |                    | appear between the |
|                    |                    |                    | &lt;MASH&gt;..&lt; |
|                    |                    |                    | /MASH&gt;          |
|                    |                    |                    | elements.          |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the mash   |
|                    |                    |                    | profile.           |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | mash record.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **GRAIN\_TEMP**    | Yes                | Temperature (C)    | The temperature of |
|                    |                    |                    | the grain before   |
|                    |                    |                    | adding it to the   |
|                    |                    |                    | mash in degrees    |
|                    |                    |                    | Celsius.           |
+--------------------+--------------------+--------------------+--------------------+
| **MASH\_STEPS**    | Yes                | Record Set         | Record set that    |
|                    |                    |                    | starts the list of |
|                    |                    |                    | &lt;MASH\_STEP&gt; |
|                    |                    |                    | records.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>All         |
|                    |                    |                    | MASH\_STEP records |
|                    |                    |                    | should appear      |
|                    |                    |                    | between the        |
|                    |                    |                    | &lt;MASH\_STEPS&gt |
|                    |                    |                    | ;                  |
|                    |                    |                    | …                  |
|                    |                    |                    | &lt;/MASH\_STEPS&g |
|                    |                    |                    | t;                 |
|                    |                    |                    | pair.              |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Notes associated   |
|                    |                    |                    | with this profile  |
|                    |                    |                    | – may be           |
|                    |                    |                    | multiline.         |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_TEMP**      | No                 | Temperature (C)    | Grain tun          |
|                    |                    |                    | temperature – may  |
|                    |                    |                    | be used to adjust  |
|                    |                    |                    | the infusion       |
|                    |                    |                    | temperature for    |
|                    |                    |                    | equipment if the   |
|                    |                    |                    | program supports   |
|                    |                    |                    | it.<span           |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Measured in |
|                    |                    |                    | degrees C.         |
+--------------------+--------------------+--------------------+--------------------+
| **SPARGE\_TEMP**   | No                 | Temperature (C)    | Temperature of the |
|                    |                    |                    | sparge water used  |
|                    |                    |                    | in degrees         |
|                    |                    |                    | Celsius.           |
+--------------------+--------------------+--------------------+--------------------+
| **PH**             | No                 | Floating Point     | PH of the sparge.  |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_WEIGHT**    | No                 | Weight (Kg)        | Weight of the mash |
|                    |                    |                    | tun in kilograms   |
+--------------------+--------------------+--------------------+--------------------+
| **TUN\_SPECIFIC\_H | No                 | Floating Point     | Specific heat of   |
| EAT**              |                    |                    | the tun material   |
|                    |                    |                    | in calories per    |
|                    |                    |                    | gram-degree C.     |
+--------------------+--------------------+--------------------+--------------------+
| **EQUIP\_ADJUST**  | No                 | Boolean            | If TRUE, mash      |
|                    |                    |                    | infusion and       |
|                    |                    |                    | decoction          |
|                    |                    |                    | calculations       |
|                    |                    |                    | should take into   |
|                    |                    |                    | account the        |
|                    |                    |                    | temperature        |
|                    |                    |                    | effects of the     |
|                    |                    |                    | equipment (tun     |
|                    |                    |                    | specific heat and  |
|                    |                    |                    | tun weight).<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If FALSE,   |
|                    |                    |                    | the tun is assumed |
|                    |                    |                    | to be              |
|                    |                    |                    | pre-heated.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Default is  |
|                    |                    |                    | FALSE.             |
+--------------------+--------------------+--------------------+--------------------+

****

 

****

 

**Sample Single Step Infusion Mash**

&lt;MASH&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Single Step Infusion,
68 C&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;GRAIN\_TEMP&gt;22.0&lt;/GRAIN\_TEMP&gt;</span>

&lt;MASH\_STEPS&gt;

<span style="mso-spacerun:yes">    </span>&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes">   </span><span
style="mso-tab-count:1">         </span>&lt;NAME&gt;Conversion Step, 68C
&lt;/NAME&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span><span lang="FR"
style="mso-ansi-language:
FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">    
</span>&lt;TYPE&gt;Infusion&lt;/TYPE&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">    
</span></span>&lt;STEP\_TEMP&gt;68.0&lt;/STEP\_TEMP&gt;

<span style="mso-tab-count:1">           
</span>&lt;STEP\_TIME&gt;60.0&lt;/STEP\_TIME&gt;

&lt;INFUSE\_AMOUNT&gt;10.0&lt;/INFUSE\_AMOUNT&gt;

<span style="mso-spacerun:yes">      </span>&lt;/MASH\_STEP&gt;

<span style="mso-spacerun:yes"> </span>&lt;/MASH\_STEPS&gt;

&lt;/MASH&gt;

 

**Sample Two Step Temperature Mash**

&lt;MASH&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Two Step Temperature,
68C &lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;GRAIN\_TEMP&gt;22.0&lt;/GRAIN\_TEMP&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;TUN\_TEMP&gt;22.0&lt;/TUN\_TEMP&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes"> </span>&lt;SPARGE\_TEMP&gt;78.0&lt;/SPARGE\_TEMP&gt;</span>

&lt;MASH\_STEPS&gt;

<span style="mso-spacerun:yes">    </span>&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes">   </span><span
style="mso-tab-count:1">         </span>&lt;NAME&gt;Protein
Rest&lt;/NAME&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span>&lt;TYPE&gt;Temperature&lt;/TYPE&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">    
</span>&lt;STEP\_TEMP&gt;49.0&lt;/STEP\_TEMP&gt;

<span style="mso-tab-count:1">           
</span>&lt;STEP\_TIME&gt;20.0&lt;/STEP\_TIME&gt;

<span style="mso-tab-count:1">           
</span>&lt;RAMP\_TIME&gt;10.0&lt;RAMP\_TIME&gt;

<span style="mso-tab-count:1">           
</span>&lt;INFUSE\_AMOUNT&gt;15.0&lt;/INFUSE\_AMOUNT&gt;

<span style="mso-spacerun:yes">      </span>&lt;/MASH\_STEP&gt;

<span style="mso-spacerun:yes">    </span>&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes">   </span><span
style="mso-tab-count:1">         </span>&lt;NAME&gt;Conversion Step, 68
C&lt;/NAME&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span>&lt;TYPE&gt;Temperature&lt;/TYPE&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">    
</span>&lt;STEP\_TEMP&gt;68.0&lt;/STEP\_TEMP&gt;

<span style="mso-tab-count:1">           
</span>&lt;RAMP\_TIME&gt;20.0&lt;RAMP\_TIME&gt;

<span style="mso-tab-count:1">           
</span>&lt;STEP\_TIME&gt;60.0&lt;/STEP\_TIME&gt;

<span style="mso-spacerun:yes">      </span>&lt;/MASH\_STEP&gt;

<span style="mso-spacerun:yes"> </span>&lt;/MASH\_STEPS&gt;

&lt;/MASH&gt;

****

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Recipe</span>

A recipe record denotes a single recipe.<span style="mso-spacerun:yes"> 
</span>A recipe record may use records from any of the previously
described record formats to specify ingredients and other data.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **RECIPE**         | Yes                | Record             | Starts a RECIPE.   |
+--------------------+--------------------+--------------------+--------------------+
| **NAME**           | Yes                | Text               | Name of the        |
|                    |                    |                    | recipe.            |
+--------------------+--------------------+--------------------+--------------------+
| **VERSION**        | Yes                | Integer            | Version of the     |
|                    |                    |                    | recipe             |
|                    |                    |                    | record.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Should      |
|                    |                    |                    | always be “1” for  |
|                    |                    |                    | this version of    |
|                    |                    |                    | the XML standard.  |
+--------------------+--------------------+--------------------+--------------------+
| **TYPE**           | Yes                | List               | May be one of      |
|                    |                    |                    | “Extract”,         |
|                    |                    |                    | “Partial Mash” or  |
|                    |                    |                    | “All Grain”        |
+--------------------+--------------------+--------------------+--------------------+
| **STYLE**          | Yes                | Style Record       | The style of the   |
|                    |                    |                    | beer this recipe   |
|                    |                    |                    | is associated      |
|                    |                    |                    | with.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>All of the  |
|                    |                    |                    | required items for |
|                    |                    |                    | a valid style      |
|                    |                    |                    | should be between  |
|                    |                    |                    | the                |
|                    |                    |                    | &lt;STYLE&gt;…&lt; |
|                    |                    |                    | /STYLE&gt;         |
|                    |                    |                    | tags.              |
+--------------------+--------------------+--------------------+--------------------+
| **EQUIPMENT**      | No                 | Equipment Record   | An equipment       |
|                    |                    |                    | record is          |
|                    |                    |                    | optional.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>If included |
|                    |                    |                    | the BATCH\_SIZE    |
|                    |                    |                    | and BOIL\_SIZE in  |
|                    |                    |                    | the equipment      |
|                    |                    |                    | record must match  |
|                    |                    |                    | the values in this |
|                    |                    |                    | recipe record.     |
+--------------------+--------------------+--------------------+--------------------+
| **BREWER**         | Yes                | Text               | Name of the brewer |
+--------------------+--------------------+--------------------+--------------------+
| **ASST\_BREWER**   | No                 | Text               | Optional name of   |
|                    |                    |                    | the assistant      |
|                    |                    |                    | brewer             |
+--------------------+--------------------+--------------------+--------------------+
| **BATCH\_SIZE**    | Yes                | Volume (liters)    | Target size of the |
|                    |                    |                    | finished batch in  |
|                    |                    |                    | liters.            |
+--------------------+--------------------+--------------------+--------------------+
| **BOIL\_SIZE**     | Yes                | Volume (liters)    | Starting size for  |
|                    |                    |                    | the main boil of   |
|                    |                    |                    | the wort in        |
|                    |                    |                    | liters.            |
+--------------------+--------------------+--------------------+--------------------+
| **BOIL\_TIME**     | Yes                | Time in minutes    | The total time to  |
|                    |                    |                    | boil the wort in   |
|                    |                    |                    | minutes.           |
+--------------------+--------------------+--------------------+--------------------+
| **EFFICIENCY**     | Conditional        | Percentage         | The percent        |
|                    |                    |                    | brewhouse          |
|                    |                    |                    | efficiency to be   |
|                    |                    |                    | used for           |
|                    |                    |                    | estimating the     |
|                    |                    |                    | starting gravity   |
|                    |                    |                    | of the beer. <span |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">  </span>No |
|                    |                    |                    | t                  |
|                    |                    |                    | required for       |
|                    |                    |                    | “Extract” recipes, |
|                    |                    |                    | but is required    |
|                    |                    |                    | for “Partial Mash” |
|                    |                    |                    | and “All Grain”    |
|                    |                    |                    | recipes.           |
+--------------------+--------------------+--------------------+--------------------+
| **HOPS**           | Yes                | Hops Record Set    | Zero or more HOP   |
|                    |                    |                    | ingredient records |
|                    |                    |                    | may appear between |
|                    |                    |                    | the                |
|                    |                    |                    | &lt;HOPS&gt;…&lt;/ |
|                    |                    |                    | HOPS&gt;           |
|                    |                    |                    | tags.              |
+--------------------+--------------------+--------------------+--------------------+
| **FERMENTABLES**   | Yes                | Fermentables       | Zero or more       |
|                    |                    | Record Set         | FERMENTABLE        |
|                    |                    |                    | ingredients may    |
|                    |                    |                    | appear between the |
|                    |                    |                    | &lt;FERMENTABLES&g |
|                    |                    |                    | t;                 |
|                    |                    |                    | …                  |
|                    |                    |                    | &lt;/FERMENTABLES& |
|                    |                    |                    | gt;                |
|                    |                    |                    | tags.              |
+--------------------+--------------------+--------------------+--------------------+
| **MISCS**          | Yes                | Miscs Record Set   | Zero or more MISC  |
|                    |                    |                    | records may appear |
|                    |                    |                    | between            |
|                    |                    |                    | &lt;MISCS&gt; …    |
|                    |                    |                    | &lt;/MISCS&gt;     |
+--------------------+--------------------+--------------------+--------------------+
| **YEASTS**         | Yes                | Yeasts Record Set  | Zero or more YEAST |
|                    |                    |                    | records may appear |
|                    |                    |                    | between            |
|                    |                    |                    | &lt;YEASTS&gt; …   |
|                    |                    |                    | &lt;/YEASTS&gt;    |
+--------------------+--------------------+--------------------+--------------------+
| **WATERS**         | Yes                | Waters Record Set  | Zero or more WATER |
|                    |                    |                    | records may appear |
|                    |                    |                    | between            |
|                    |                    |                    | &lt;WATERS&gt; …   |
|                    |                    |                    | &lt;/WATERS&gt;    |
+--------------------+--------------------+--------------------+--------------------+
| **MASH**           | Yes                | Mash Profile       | A MASH profile     |
|                    |                    |                    | record containing  |
|                    |                    |                    | one or more        |
|                    |                    |                    | MASH\_STEPs.<span  |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>NOTE: No    |
|                    |                    |                    | Mash record is     |
|                    |                    |                    | needed for         |
|                    |                    |                    | “Extract” type     |
|                    |                    |                    | brews.             |
+--------------------+--------------------+--------------------+--------------------+
| **NOTES**          | No                 | Text               | Notes associated   |
|                    |                    |                    | with this recipe – |
|                    |                    |                    | may be multiline.  |
+--------------------+--------------------+--------------------+--------------------+
| **TASTE\_NOTES**   | No                 | Text               | Tasting notes –    |
|                    |                    |                    | may be multiline.  |
+--------------------+--------------------+--------------------+--------------------+
| **TASTE\_RATING**  | No                 | Floating Point     | Number between     |
|                    |                    |                    | zero and 50.0      |
|                    |                    |                    | denoting the taste |
|                    |                    |                    | rating –           |
|                    |                    |                    | corresponds to the |
|                    |                    |                    | 50 point BJCP      |
|                    |                    |                    | rating system.     |
+--------------------+--------------------+--------------------+--------------------+
| **OG**             | No                 | Specific Gravity   | The measured       |
|                    |                    |                    | original           |
|                    |                    |                    | (pre-fermentation) |
|                    |                    |                    | specific gravity   |
|                    |                    |                    | of the beer.       |
+--------------------+--------------------+--------------------+--------------------+
| **FG**             | No                 | Specific Gravity   | The measured final |
|                    |                    |                    | gravity of the     |
|                    |                    |                    | finished beer.     |
+--------------------+--------------------+--------------------+--------------------+
| **FERMENTATION\_ST | No                 | Integer            | The number of      |
| AGES**             |                    |                    | fermentation       |
|                    |                    |                    | stages used –      |
|                    |                    |                    | typically a number |
|                    |                    |                    | between one and    |
|                    |                    |                    | three              |
+--------------------+--------------------+--------------------+--------------------+
| **PRIMARY\_AGE**   | No                 | Time (days)        | Time spent in the  |
|                    |                    |                    | primary in days    |
+--------------------+--------------------+--------------------+--------------------+
| **PRIMARY\_TEMP**  | No                 | Temperature C      | Temperature in     |
|                    |                    |                    | degrees Celsius    |
|                    |                    |                    | for the primary    |
|                    |                    |                    | fermentation.      |
+--------------------+--------------------+--------------------+--------------------+
| **SECONDARY\_AGE** | No                 | Time (days)        | Time spent in the  |
|                    |                    |                    | secondary in days. |
+--------------------+--------------------+--------------------+--------------------+
| **SECONDARY\_TEMP* | No                 | Temperature (C )   | Temperature in     |
| *                  |                    |                    | degrees Celsius    |
|                    |                    |                    | for the secondary  |
|                    |                    |                    | fermentation.      |
+--------------------+--------------------+--------------------+--------------------+
| **TERTIARY\_AGE**  | No                 | Time (days)        | Time spent in the  |
|                    |                    |                    | third fermenter in |
|                    |                    |                    | days.              |
+--------------------+--------------------+--------------------+--------------------+
| **TERTIARY\_TEMP** | No                 | Temperature C      | Temperature in the |
|                    |                    |                    | tertiary           |
|                    |                    |                    | fermenter.         |
+--------------------+--------------------+--------------------+--------------------+
| **AGE**            | No                 | Time (days)        | The time to age    |
|                    |                    |                    | the beer in days   |
|                    |                    |                    | after bottling.    |
+--------------------+--------------------+--------------------+--------------------+
| **AGE\_TEMP**      | No                 | Temperature C      | Temperature for    |
|                    |                    |                    | aging the beer     |
|                    |                    |                    | after bottling.    |
+--------------------+--------------------+--------------------+--------------------+
| **DATE**           | No                 | Text               | Date brewed in a   |
|                    |                    |                    | easily             |
|                    |                    |                    | recognizable       |
|                    |                    |                    | format such as “3  |
|                    |                    |                    | Dec 04”.           |
+--------------------+--------------------+--------------------+--------------------+
| **CARBONATION**    | No                 | Volumes of CO2     | Floating point     |
|                    |                    |                    | value              |
|                    |                    |                    | corresponding to   |
|                    |                    |                    | the target volumes |
|                    |                    |                    | of CO2 used to     |
|                    |                    |                    | carbonate this     |
|                    |                    |                    | beer.              |
+--------------------+--------------------+--------------------+--------------------+
| **FORCED\_CARBONAT | No                 | Boolean            | TRUE if the batch  |
| ION**              |                    |                    | was force          |
|                    |                    |                    | carbonated using   |
|                    |                    |                    | CO2 pressure,      |
|                    |                    |                    | FALSE if the batch |
|                    |                    |                    | was carbonated     |
|                    |                    |                    | using a priming    |
|                    |                    |                    | agent.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Default is  |
|                    |                    |                    | FALSE              |
+--------------------+--------------------+--------------------+--------------------+
| **PRIMING\_SUGAR\_ | No                 | Text               | Text describing    |
| NAME**             |                    |                    | the priming agent  |
|                    |                    |                    | such as “Honey” or |
|                    |                    |                    | “Corn Sugar” –     |
|                    |                    |                    | used only if this  |
|                    |                    |                    | is not a forced    |
|                    |                    |                    | carbonation        |
+--------------------+--------------------+--------------------+--------------------+
| **CARBONATION\_TEM | No                 | Temperature        | The temperature    |
| P**                |                    | (degrees C)        | for either         |
|                    |                    |                    | bottling or forced |
|                    |                    |                    | carbonation.       |
+--------------------+--------------------+--------------------+--------------------+
| **PRIMING\_SUGAR\_ | No                 | Floating point     | Factor used to     |
| EQUIV**            |                    |                    | convert this       |
|                    |                    |                    | priming agent to   |
|                    |                    |                    | an equivalent      |
|                    |                    |                    | amount of corn     |
|                    |                    |                    | sugar for a        |
|                    |                    |                    | bottled            |
|                    |                    |                    | scenario.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For         |
|                    |                    |                    | example, “Dry Malt |
|                    |                    |                    | Extract” would     |
|                    |                    |                    | have a value of    |
|                    |                    |                    | 1.4 because it     |
|                    |                    |                    | requires 1.4 times |
|                    |                    |                    | as much DME as     |
|                    |                    |                    | corn sugar to      |
|                    |                    |                    | carbonate.<span    |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>To          |
|                    |                    |                    | calculate the      |
|                    |                    |                    | amount of DME      |
|                    |                    |                    | needed, the        |
|                    |                    |                    | program can        |
|                    |                    |                    | calculate the      |
|                    |                    |                    | amount of corn     |
|                    |                    |                    | sugar needed and   |
|                    |                    |                    | then multiply by   |
|                    |                    |                    | this factor.       |
+--------------------+--------------------+--------------------+--------------------+
| **KEG\_PRIMING\_FA | No                 | Floating point     | Used to factor in  |
| CTOR**             |                    |                    | the smaller amount |
|                    |                    |                    | of sugar needed    |
|                    |                    |                    | for large          |
|                    |                    |                    | containers.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For         |
|                    |                    |                    | example, this      |
|                    |                    |                    | might be 0.5 for a |
|                    |                    |                    | typical 5 gallon   |
|                    |                    |                    | keg since          |
|                    |                    |                    | naturally priming  |
|                    |                    |                    | a keg requires     |
|                    |                    |                    | about 50% as much  |
|                    |                    |                    | sugar as priming   |
|                    |                    |                    | bottles.           |
+--------------------+--------------------+--------------------+--------------------+

****

 

****

 

**Sample Complete Recipe File in XML - Dry Stout**

****

 

<span style="color:black">&lt;?xml version="1.0"
encoding="ISO-8859-1"?&gt;</span>

<span style="color:black">&lt;RECIPES&gt;</span>

**<span style="mso-spacerun:yes">  </span>&lt;**RECIPE&gt;

<span style="mso-spacerun:yes"> </span><span
style="mso-spacerun:yes"> </span>&lt;NAME&gt;Dry Stout&lt;/NAME&gt;

<span style="mso-spacerun:yes">  </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes">   </span>&lt;TYPE&gt;All
Grain&lt;/TYPE&gt;

<span style="mso-spacerun:yes">   </span>&lt;BREWER&gt;Brad
Smith&lt;/BREWER&gt;

<span style="mso-spacerun:yes">  
</span>&lt;BATCH\_SIZE&gt;18.93&lt;/BATCH\_SIZE&gt;

<span style="mso-spacerun:yes">   
</span>&lt;BOIL\_SIZE&gt;20.82&lt;/BOIL\_SIZE&gt;

<span style="mso-spacerun:yes">   
</span>&lt;BOIL\_TIME&gt;60.0&lt;/BOIL\_TIME&gt;

<span style="mso-spacerun:yes">   
</span>&lt;EFFICIENCY&gt;72.0&lt;/EFFICIENCY&gt;

<span style="mso-spacerun:yes">   </span>&lt;TASTE\_NOTES&gt;Nice dry
Irish stout with a warm body but low starting gravity much like the
famous drafts.&lt;/TASTE\_NOTES&gt;

<span style="mso-spacerun:yes">   </span>&lt;RATING&gt;41&lt;/RATING&gt;

<span style="mso-spacerun:yes">    </span>&lt;DATE&gt;3 Jan
04&lt;/DATE&gt;

<span style="mso-spacerun:yes">    </span>&lt;OG&gt;1.036&lt;/OG&gt;

<span style="mso-spacerun:yes">    </span>&lt;FG&gt;1.012&lt;/FG&gt;

<span style="mso-spacerun:yes">   
</span>&lt;CARBONATION&gt;2.1&lt;/CARBONATION&gt;

<span style="mso-spacerun:yes">   
</span>&lt;CARBONATION\_USED&gt;Kegged&lt;/CARBONATION\_USED&gt;

<span style="mso-spacerun:yes">    </span>&lt;AGE&gt;24.0&lt;/AGE&gt;

<span style="mso-spacerun:yes">   
</span>&lt;AGE\_TEMP&gt;17.0&lt;/AGE\_TEMP&gt;

<span style="mso-spacerun:yes">    </span><span lang="FR"
style="mso-ansi-language:FR">&lt;FERMENTATION\_STAGES&gt;2&lt;/FERMENTATION\_STAGES&gt;</span>

<span lang="FR" style="mso-ansi-language:FR"><span
style="mso-spacerun:yes">   </span>&lt;STYLE&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span style="mso-spacerun:yes"> </span></span>&lt;NAME&gt;Dry
Stout&lt;/NAME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CATEGORY&gt;Stout&lt;/CATEGORY&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CATEGORY\_NUMBER&gt;16&lt;/CATEGORY\_NUMBER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;STYLE\_LETTER&gt;A&lt;/STYLE\_LETTER&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;STYLE\_GUIDE&gt;BJCP&lt;/STYLE\_GUIDE&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span></span>&lt;TYPE&gt;Ale&lt;/TYPE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MIN&gt;1.035&lt;/OG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;OG\_MAX&gt;1.050&lt;/OG\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MIN&gt;1.007&lt;/FG\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FG\_MAX&gt;1.011&lt;/FG\_MAX&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;IBU\_MIN&gt;30.0&lt;/IBU\_MIN&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span>&lt;IBU\_MAX&gt;50.0&lt;/IBU\_MAX&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span></span>&lt;COLOR\_MIN&gt;35.0&lt;/COLOR\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COLOR\_MAX&gt;200.0&lt;/COLOR\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;ABV\_MIN&gt;3.2&lt;/ABV\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;ABV\_MAX&gt;5.5&lt;/ABV\_MAX&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CARB\_MIN&gt;1.6&lt;/CARB\_MIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CARB\_MAX&gt;2.1&lt;/CARB\_MAX&gt;

<span style="mso-spacerun:yes">              </span>&lt;NOTES&gt;Famous
Irish Stout.<span style="mso-spacerun:yes">  </span>Dry, roasted, almost
coffee like flavor.<span style="mso-spacerun:yes">  </span>Often soured
with pasteurized sour beer.<span style="mso-spacerun:yes">  </span>Full
body perception due to flaked barley, though starting gravity may be
low.<span style="mso-spacerun:yes">  </span>Dry roasted
flavor.&lt;/NOTES&gt;

<span style="mso-spacerun:yes">  </span>&lt;/STYLE&gt;

<span style="mso-spacerun:yes">  </span>&lt;HOPS&gt;

<span style="mso-spacerun:yes">  </span>&lt;HOP&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Goldings, East
Kent&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;ALPHA&gt;5.0&lt;/ALPHA&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.0638&lt;/AMOUNT&gt;

&lt;USE&gt;Boil&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;TIME&gt;60.0&lt;/TIME&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Great all purpose UK
hop for ales, stouts, porters&lt;/NOTES&gt;

&lt;/HOP&gt;

<span style="mso-spacerun:yes">    </span>&lt;/HOPS&gt;

<span style="mso-spacerun:yes">    </span>&lt;FERMENTABLES&gt;

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Pale Malt (2 row)
UK&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;2.27&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Grain&lt;/TYPE&gt;

&lt;YIELD&gt;78.0&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;3.0&lt;/COLOR&gt;

<span style="mso-spacerun:yes"> </span>&lt;ORIGIN&gt;United
Kingdom&lt;/ORIGIN&gt;

<span style="mso-spacerun:yes"> </span>&lt;SUPPLIER&gt;Fussybrewer
Malting&lt;/SUPPLIER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;All purpose base
malt for English styles&lt;/NOTES&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COARSE\_FINE\_DIFF&gt;1.5&lt;/COARSE\_FINE\_DIFF&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MOISTURE&gt;4.0&lt;/MOISTURE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;DIASTATIC\_POWER&gt;45.0&lt;/DISASTATIC\_POWER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PROTEIN&gt;10.2&lt;/PROTEIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAX\_IN\_BATCH&gt;100.0&lt;/MAX\_IN\_BATCH&gt;

&lt;/FERMENTABLE&gt;

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Barley,
Flaked&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.91&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Grain&lt;/TYPE&gt;

&lt;YIELD&gt;70.0&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;2.0&lt;/COLOR&gt;

<span style="mso-spacerun:yes"> </span>&lt;ORIGIN&gt;United
Kingdom&lt;/ORIGIN&gt;

<span style="mso-spacerun:yes"> </span>&lt;SUPPLIER&gt;Fussybrewer
Malting&lt;/SUPPLIER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Adds body to porters
and stouts, must be mashed&lt;/NOTES&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COARSE\_FINE\_DIFF&gt;1.5&lt;/COARSE\_FINE\_DIFF&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MOISTURE&gt;9.0&lt;/MOISTURE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;DIASTATIC\_POWER&gt;0.0&lt;/DISASTATIC\_POWER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PROTEIN&gt;13.2&lt;/PROTEIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAX\_IN\_BATCH&gt;20.0&lt;/MAX\_IN\_BATCH&gt;

<span
style="mso-spacerun:yes"> </span>&lt;RECOMMEND\_MASH&gt;TRUE&lt;/RECOMMEND\_MASH&gt;

&lt;/FERMENTABLE&gt;

&lt;FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Black
Barley&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.45&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Grain&lt;/TYPE&gt;

&lt;YIELD&gt;78.0&lt;/YIELD&gt;

<span style="mso-spacerun:yes"> </span>&lt;COLOR&gt;500.0&lt;/COLOR&gt;

<span style="mso-spacerun:yes"> </span>&lt;ORIGIN&gt;United
Kingdom&lt;/ORIGIN&gt;

<span style="mso-spacerun:yes"> </span>&lt;SUPPLIER&gt;Fussybrewer
Malting&lt;/SUPPLIER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Unmalted roasted
barley for stouts, porters&lt;/NOTES&gt;

<span
style="mso-spacerun:yes"> </span>&lt;COARSE\_FINE\_DIFF&gt;1.5&lt;/COARSE\_FINE\_DIFF&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MOISTURE&gt;5.0&lt;/MOISTURE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;DIASTATIC\_POWER&gt;0.0&lt;/DISASTATIC\_POWER&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PROTEIN&gt;13.2&lt;/PROTEIN&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAX\_IN\_BATCH&gt;10.0&lt;/MAX\_IN\_BATCH&gt;

&lt;/FERMENTABLE&gt;

<span style="mso-spacerun:yes"> </span>&lt;/FERMENTABLES&gt;

<span style="mso-spacerun:yes"> </span>&lt;MISCS&gt;

<span style="mso-spacerun:yes"> </span>&lt;MISC&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Irish
Moss&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;TYPE&gt;Fining&lt;/TYPE&gt;

<span style="mso-spacerun:yes"> </span>&lt;USE&gt;Boil&lt;/USE&gt;

<span style="mso-spacerun:yes"> </span>&lt;TIME&gt;15.0&lt;/TIME&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.010&lt;/AMOUNT&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt;Used as a clarifying
agent during the last few minutes of the boil&lt;/NOTES&gt;

&lt;/MISC&gt;

<span style="mso-spacerun:yes"> </span>&lt;/MISCS&gt;

<span style="mso-spacerun:yes"> </span>&lt;WATERS&gt;

&lt;WATER&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Burton on Trent,
UK&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span>&lt;VERSION&gt;1&lt;/VERSION&gt;

<span style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;20.0&lt;/AMOUNT&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CALCIUM&gt;295.0&lt;/CALCIUM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MAGNESIUM&gt;45.0&lt;/MAGNESIUM&gt;

<span style="mso-spacerun:yes"> </span>&lt;SODIUM&gt;55.0&lt;/SODIUM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;SULFATE&gt;725.0&lt;/SULFATE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;CHLORIDE&gt;25.0&lt;/CHLORIDE&gt;

<span
style="mso-spacerun:yes"> </span>&lt;BICARBONATE&gt;300.0&lt;/BICARBONATE&gt;

<span style="mso-spacerun:yes"> </span>&lt;PH&gt;8.0&lt;/PH&gt;

<span style="mso-spacerun:yes"> </span>&lt;NOTES&gt; Use for distinctive
pale ales strongly hopped.<span style="mso-spacerun:yes">  </span>Very
hard water accentuates the hops flavor. Example: Bass Ale

&lt;/NOTES&gt;

&lt;/WATER&gt;

<span style="mso-spacerun:yes">  </span>&lt;/WATERS&gt;

<span style="mso-spacerun:yes"> </span>&lt;YEASTS&gt;

&lt;YEAST&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Irish
Ale&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;TYPE&gt;Ale&lt;/TYPE&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span></span>&lt;FORM&gt;Liquid&lt;/FORM&gt;

<span
style="mso-spacerun:yes"> </span>&lt;AMOUNT&gt;0.250&lt;/AMOUNT&gt;

&lt;LABORATORY&gt;Wyeast Labs&lt;/LABORATORY&gt;

<span
style="mso-spacerun:yes"> </span>&lt;PRODUCT\_ID&gt;1084&lt;/PRODUCT\_ID&gt;

<span
style="mso-spacerun:yes"> </span>&lt;MIN\_TEMPERATURE&gt;16.7&lt;/MIN\_TEMPERATURE&gt;

&lt;MAX\_TEMPERATURE&gt;22.2&lt;/MAX\_TEMPERATURE&gt;

&lt;ATTENUATION&gt;73.0&lt;/ATTENUATION&gt;

&lt;NOTES&gt;Dry, fruity flavor characteristic of stouts.<span
style="mso-spacerun:yes">  </span>Full bodied, dry, clean flavor.
&lt;/NOTES&gt;

<span style="mso-spacerun:yes"> </span>&lt;BEST\_FOR&gt;Irish Dry
Stouts&lt;/BEST\_FOR&gt;

<span
style="mso-spacerun:yes"> </span>&lt;FLOCCULATION&gt;Medium&lt;/FLOCCULATION&gt;

&lt;/YEAST&gt;

<span style="mso-spacerun:yes"> </span>&lt;/YEASTS&gt;

&lt;MASH&gt;

<span style="mso-spacerun:yes"> </span>&lt;NAME&gt;Single Step Infusion,
68 C&lt;/NAME&gt;

<span style="mso-spacerun:yes"> </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span
style="mso-spacerun:yes"> </span>&lt;GRAIN\_TEMP&gt;22.0&lt;/GRAIN\_TEMP&gt;</span>

&lt;MASH\_STEPS&gt;

<span style="mso-spacerun:yes">    </span>&lt;MASH\_STEP&gt;

<span style="mso-spacerun:yes">   </span><span
style="mso-tab-count:1">         </span>&lt;NAME&gt;Conversion Step, 68C
&lt;/NAME&gt;

<span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:1">     </span><span lang="FR"
style="mso-ansi-language:FR">&lt;VERSION&gt;1&lt;/VERSION&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:
1">     </span>&lt;TYPE&gt;Infusion&lt;/TYPE&gt;</span>

<span lang="FR" style="mso-ansi-language:
FR"><span style="mso-spacerun:yes">       </span><span
style="mso-tab-count:
1">     </span></span>&lt;STEP\_TEMP&gt;68.0&lt;/STEP\_TEMP&gt;

<span style="mso-tab-count:1">           
</span>&lt;STEP\_TIME&gt;60.0&lt;/STEP\_TIME&gt;

&lt;INFUSE\_AMOUNT&gt;10.0&lt;/INFUSE\_AMOUNT&gt;

<span style="mso-spacerun:yes">      </span>&lt;/MASH\_STEP&gt;

<span style="mso-spacerun:yes"> </span>&lt;/MASH\_STEPS&gt;

&lt;/MASH&gt;

&lt;/RECIPE&gt;

&lt;/RECIPES&gt;

****

 

**<span style="font-size:12.0pt;
font-family:&quot;Times New Roman&quot;;mso-fareast-font-family:&quot;Times New Roman&quot;;
mso-ansi-language:EN-US;mso-fareast-language:EN-US;mso-bidi-language:AR-SA">\
</span>**
***<span style="font-size:26.0pt">Appendix A</span>***

***<span style="font-size:26.0pt">Optional Extensions for</span>***

***<span style="font-size:26.0pt">BeerXML Display</span>***

<span style="font-size:16.0pt;mso-bidi-font-size:12.0pt"></span>
================================================================

 

<span style="font-size:16.0pt;mso-bidi-font-size:12.0pt">Purpose</span>
=======================================================================

This document describes optional extensions that may be exported by a
particular beer program to enhance the easy display of data and
construction of XML style sheets.

 

These options are NOT required and all may not be supported by a
particular program.<span style="mso-spacerun:yes">  </span>Where
implemented, these tags provide a consistent method for display
only.<span style="mso-spacerun:yes">  </span>None of these values should
be used for import as the display value may be rounded from the true
value.

 

 

<span style="font-size:16.0pt;mso-bidi-font-size:12.0pt">Standards</span>
=========================================================================

All standards of the original BeerXML description also apply here with
the exception of units – all fields that are defined for display only
may also use a unit tag after them.<span style="mso-spacerun:yes">   
</span>For example “3.45 gal” is an acceptable value.<span
style="mso-spacerun:yes">  </span>For consistency, the recognized unit
tags are described below.

Units
-----

The following units are allowed and may be used interchangeably.<span
style="mso-spacerun:yes">  </span>However, only units of the appropriate
type may be used for a given value.<span style="mso-spacerun:yes"> 
</span>For example "volume" units may not be used for "Weight" fields.

### Weight Units

**kg** <span style="mso-spacerun:yes"> </span>- Kilograms

**g -** Grams

**oz -** Ounces

**lb** – Pounds

### Volume Units

**tsp –** <span style="mso-bidi-font-weight:bold">Teaspoons</span>

**tblsp**<span style="mso-bidi-font-weight:bold"> – Tablespoons</span>

**oz –** Ounces (US)

**cup –** Cups (US)

**pt –** Pints (US)

**qt –** Quarts (US)

**ml -** Milliliters

**l –** Liters

### Temperature Units

**F –** <span style="mso-bidi-font-weight:bold">Degrees
</span>Fahrenheit

**C –** Degrees Celsius

### Time Units

**min -** Minutes

**hour -** Hours

**day –** Days

**week –** Weeks

### Color Units

**srm –** SRM Color

**ebc –** EBC Color

**L –** Degrees lovibond.

### Specific Gravity Units

**sg –** The relative gravity by weight when compared to water.<span
style="mso-spacerun:yes">  </span>For example “1.035 sg”

**plato –** Gravity measured in degrees plato

 

<span style="font-size:26.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Hop Extensions</span>

The following extensions may be used within HOP records for the purpose
of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_AMOUNT* | No                 | Text               | The amount of hops |
| *                  |                    |                    | in this record     |
|                    |                    |                    | along with the     |
|                    |                    |                    | units formatted    |
|                    |                    |                    | for easy display   |
|                    |                    |                    | in the current     |
|                    |                    |                    | user defined       |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “100 g” or “1.5    |
|                    |                    |                    | oz”.               |
+--------------------+--------------------+--------------------+--------------------+
| **INVENTORY**      | No                 | Text               | Amount in          |
|                    |                    |                    | inventory for this |
|                    |                    |                    | item along with    |
|                    |                    |                    | the units – for    |
|                    |                    |                    | example “10.0 oz”  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TIME**  | No                 | Text               | Time displayed in  |
|                    |                    |                    | minutes for all    |
|                    |                    |                    | uses except for    |
|                    |                    |                    | the dry hop which  |
|                    |                    |                    | is in days.<span   |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “60 min”, “3       |
|                    |                    |                    | days”.             |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Fermentable Extensions</span>

The following extensions may be used within FERMENTABLE records for the
purpose of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_AMOUNT* | No                 | Text               | The amount of      |
| *                  |                    |                    | fermentables in    |
|                    |                    |                    | this record along  |
|                    |                    |                    | with the units     |
|                    |                    |                    | formatted for easy |
|                    |                    |                    | display in the     |
|                    |                    |                    | current user       |
|                    |                    |                    | defined            |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.5 lbs” or “2.1  |
|                    |                    |                    | kg”.               |
+--------------------+--------------------+--------------------+--------------------+
| **POTENTIAL**      | No                 | Specific Gravity   | The yield of the   |
|                    |                    |                    | fermentable        |
|                    |                    |                    | converted to       |
|                    |                    |                    | specific gravity   |
|                    |                    |                    | units for          |
|                    |                    |                    | display.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.036” or “1.040” |
|                    |                    |                    | might be valid     |
|                    |                    |                    | potentials.        |
+--------------------+--------------------+--------------------+--------------------+
| **INVENTORY**      | No                 | Text               | Amount in          |
|                    |                    |                    | inventory for this |
|                    |                    |                    | item along with    |
|                    |                    |                    | the units – for    |
|                    |                    |                    | example “10.0 lb”  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_COLOR** | No                 | Text               | Color in user      |
|                    |                    |                    | defined color      |
|                    |                    |                    | units along with   |
|                    |                    |                    | the unit           |
|                    |                    |                    | identified – for   |
|                    |                    |                    | example “200L” or  |
|                    |                    |                    | “40 ebc”           |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Misc Extensions</span>

The following extensions may be used within MISC records for the purpose
of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_AMOUNT* | No                 | Text               | The amount of the  |
| *                  |                    |                    | item in this       |
|                    |                    |                    | record along with  |
|                    |                    |                    | the units          |
|                    |                    |                    | formatted for easy |
|                    |                    |                    | display in the     |
|                    |                    |                    | current user       |
|                    |                    |                    | defined            |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.5 lbs” or “2.1  |
|                    |                    |                    | kg”.               |
+--------------------+--------------------+--------------------+--------------------+
| **INVENTORY**      | No                 | Text               | Amount in          |
|                    |                    |                    | inventory for this |
|                    |                    |                    | item along with    |
|                    |                    |                    | the units – for    |
|                    |                    |                    | example “10.0 lb”  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TIME**  | No                 | Text               | Time in            |
|                    |                    |                    | appropriate units  |
|                    |                    |                    | along with the     |
|                    |                    |                    | units as in “10    |
|                    |                    |                    | min” or “3 days”.  |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Yeast Extensions</span>

The following extensions may be used within YEAST records for the
purpose of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_AMOUNT* | No                 | Text               | The amount of      |
| *                  |                    |                    | yeast or starter   |
|                    |                    |                    | in this record     |
|                    |                    |                    | along with the     |
|                    |                    |                    | units formatted    |
|                    |                    |                    | for easy display   |
|                    |                    |                    | in the current     |
|                    |                    |                    | user defined       |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.5 oz” or “100   |
|                    |                    |                    | g”.                |
+--------------------+--------------------+--------------------+--------------------+
| **DISP\_MIN\_TEMP* | No                 | Text               | Minimum            |
| *                  |                    |                    | fermentation       |
|                    |                    |                    | temperature        |
|                    |                    |                    | converted to       |
|                    |                    |                    | current user units |
|                    |                    |                    | along with the     |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “54.0 F” or “24.2  |
|                    |                    |                    | C”                 |
+--------------------+--------------------+--------------------+--------------------+
| **DISP\_MAX\_TEMP* | No                 | Text               | Maximum            |
| *                  |                    |                    | fermentation       |
|                    |                    |                    | temperature        |
|                    |                    |                    | converted to       |
|                    |                    |                    | current user units |
|                    |                    |                    | along with the     |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “54.0 F” or “24.2  |
|                    |                    |                    | C”                 |
+--------------------+--------------------+--------------------+--------------------+
| **INVENTORY**      | No                 | Text               | Amount in          |
|                    |                    |                    | inventory for this |
|                    |                    |                    | hop along with the |
|                    |                    |                    | units – for        |
|                    |                    |                    | example “10.0      |
|                    |                    |                    | pkgs”              |
+--------------------+--------------------+--------------------+--------------------+
| **CULTURE\_DATE**  | No                 | Text               | Date sample was    |
|                    |                    |                    | last cultured in a |
|                    |                    |                    | neutral date form  |
|                    |                    |                    | such as “10 Dec    |
|                    |                    |                    | 04”                |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Water Extensions</span>

The following extensions may be used within WATER records for the
purpose of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_AMOUNT* | No                 | Text               | The amount of      |
| *                  |                    |                    | water in this      |
|                    |                    |                    | record along with  |
|                    |                    |                    | the units          |
|                    |                    |                    | formatted for easy |
|                    |                    |                    | display in the     |
|                    |                    |                    | current user       |
|                    |                    |                    | defined            |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “5.0 gal” or “20.0 |
|                    |                    |                    | l”.                |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Style Extensions</span>

The following extensions may be used within STYLE records for the
purpose of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_OG\_MIN | No                 | Text               | Original gravity   |
| **                 |                    |                    | minimum in user    |
|                    |                    |                    | defined units such |
|                    |                    |                    | as “1.036 sg”.     |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_OG\_MAX | No                 | Text               | Original gravity   |
| **                 |                    |                    | max in user        |
|                    |                    |                    | defined units such |
|                    |                    |                    | as “1.056 sg”      |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_FG\_MIN | No                 | Text               | Final gravity      |
| **                 |                    |                    | minimum in user    |
|                    |                    |                    | defined units such |
|                    |                    |                    | as “1.010 sg”.     |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_FG\_MAX | No                 | Text               | Final gravity      |
| **                 |                    |                    | maximum in user    |
|                    |                    |                    | defined units such |
|                    |                    |                    | as “1.019 sg”.     |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_COLOR\_ | No                 | Text               | Minimum color in   |
| MIN**              |                    |                    | user defined units |
|                    |                    |                    | such as “30 srm”.  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_COLOR\_ | No                 | Text               | Maximum color in   |
| MAX**              |                    |                    | user defined units |
|                    |                    |                    | such as “20 srm”   |
+--------------------+--------------------+--------------------+--------------------+
| **OG\_RANGE**      | No                 | Text               | Original gravity   |
|                    |                    |                    | range for the      |
|                    |                    |                    | style such as      |
|                    |                    |                    | “1.030-1.040 sg”   |
+--------------------+--------------------+--------------------+--------------------+
| **FG\_RANGE**      | No                 | Text               | Final gravity      |
|                    |                    |                    | range such as      |
|                    |                    |                    | “1.010-1.015 sg”   |
+--------------------+--------------------+--------------------+--------------------+
| **IBU\_RANGE**     | No                 | Text               | Bitterness range   |
|                    |                    |                    | in IBUs such as    |
|                    |                    |                    | “10-20 IBU”        |
+--------------------+--------------------+--------------------+--------------------+
| **CARB\_RANGE**    | No                 | Text               | Carbonation range  |
|                    |                    |                    | in volumes such as |
|                    |                    |                    | “2.0-2.6 vols”     |
+--------------------+--------------------+--------------------+--------------------+
| **COLOR\_RANGE**   | No                 | Text               | Color range such   |
|                    |                    |                    | as “10-20 SRM”     |
+--------------------+--------------------+--------------------+--------------------+
| **ABV\_RANGE**     | No                 | Text               | ABV Range for this |
|                    |                    |                    | style such as      |
|                    |                    |                    | “4.5-5.5%”         |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt;color:#3366FF"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Equipment Extensions</span>

 

The following may be used with equipment records for display purposes.

 

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_BOIL\_S | Yes                | Text               | The pre-boil       |
| IZE**              |                    |                    | volume normally    |
|                    |                    |                    | used for a batch   |
|                    |                    |                    | of this size shown |
|                    |                    |                    | in display volume  |
|                    |                    |                    | units such as “5.5 |
|                    |                    |                    | gal”               |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_BATCH\_ | Yes                | Text               | The target volume  |
| SIZE**             |                    |                    | of the batch at    |
|                    |                    |                    | the start of       |
|                    |                    |                    | fermentation in    |
|                    |                    |                    | display volume     |
|                    |                    |                    | units such as “5.0 |
|                    |                    |                    | gal”               |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TUN\_VO | No                 | Text               | Volume of the mash |
| LUME**             |                    |                    | tun in display     |
|                    |                    |                    | units such as      |
|                    |                    |                    | “10.0 gal” or      |
|                    |                    |                    | “20.0 l”           |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TUN\_WE | No                 | Text               | Weight of the mash |
| IGHT**             |                    |                    | tun in display     |
|                    |                    |                    | units such as “3.0 |
|                    |                    |                    | kg” or “6.0 lb”    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TOP\_UP | No                 | Text               | The amount of top  |
| \_WATER**          |                    |                    | up water normally  |
|                    |                    |                    | added just prior   |
|                    |                    |                    | to starting        |
|                    |                    |                    | fermentation in    |
|                    |                    |                    | display volume     |
|                    |                    |                    | such as “1.0 gal”  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TRUB\_C | No                 | Text               | The amount of wort |
| HILLER\_LOSS**     |                    |                    | normally lost      |
|                    |                    |                    | during transition  |
|                    |                    |                    | from the boiler to |
|                    |                    |                    | the fermentation   |
|                    |                    |                    | vessel.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Includes    |
|                    |                    |                    | both unusable wort |
|                    |                    |                    | due to trub and    |
|                    |                    |                    | wort lost to the   |
|                    |                    |                    | chiller and        |
|                    |                    |                    | transfer           |
|                    |                    |                    | systems.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Expressed   |
|                    |                    |                    | in user units -    |
|                    |                    |                    | Ex: “1.5 qt”       |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_LAUTER\ | No                 | Text               | Amount lost to the |
| _DEADSPACE**       |                    |                    | lauter tun and     |
|                    |                    |                    | equipment          |
|                    |                    |                    | associated with    |
|                    |                    |                    | the lautering      |
|                    |                    |                    | process. Ex: “2.0  |
|                    |                    |                    | gal” or “1.0 l”    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TOP\_UP | No                 | Text               | Amount normally    |
| \_KETTLE**         |                    |                    | added to the boil  |
|                    |                    |                    | kettle before the  |
|                    |                    |                    | boil. Ex: “1.0     |
|                    |                    |                    | gal”               |
+--------------------+--------------------+--------------------+--------------------+

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Mash Extensions</span>

The following extensions may be used within MASH records for the purpose
of export and display only.

****

 

+--------------------+--------------------+--------------------+--------------------+
| **Data tag**       | **Required**       | Format             | **Description**    |
|                    |                    | ======             |                    |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_GRAIN\_ | No                 | Text               | Grain temperature  |
| TEMP**             |                    |                    | in user display    |
|                    |                    |                    | units with the     |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For         |
|                    |                    |                    | example: “72 F”.   |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TUN\_TE | No                 | Text               | Tun temperature in |
| MP**               |                    |                    | user display       |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “68 F”             |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_SPARGE\ | No                 | Text               | Sparge temperature |
| _TEMP**            |                    |                    | in user defined    |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “178 F”            |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_TUN\_WE | No                 | Text               | Tun weight in user |
| IGHT**             |                    |                    | defined units –    |
|                    |                    |                    | for example “10    |
|                    |                    |                    | lb”                |
+--------------------+--------------------+--------------------+--------------------+

 

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Mash Step Extensions</span>

The following may optionally be used in mash steps.<span
style="mso-spacerun:yes">  </span>They must appear between the
&lt;MASH\_STEP&gt; … &lt;/MASH\_STEP&gt; tags.

 

+--------------------+--------------------+--------------------+--------------------+
| **DESCRIPTION**    | No                 | Text               | Textual            |
|                    |                    |                    | description of     |
|                    |                    |                    | this step such as  |
|                    |                    |                    | “Infuse 4.5 gal of |
|                    |                    |                    | water at 170 F” –  |
|                    |                    |                    | may be either      |
|                    |                    |                    | generated by the   |
|                    |                    |                    | program or input   |
|                    |                    |                    | by the user.       |
+--------------------+--------------------+--------------------+--------------------+
| **WATER\_GRAIN\_RA | No                 | Text               | The total ratio of |
| TIO**              |                    |                    | water to grain for |
|                    |                    |                    | this step AFTER    |
|                    |                    |                    | the infusion along |
|                    |                    |                    | with the units,    |
|                    |                    |                    | usually expressed  |
|                    |                    |                    | in qt/lb or        |
|                    |                    |                    | l/kg.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Note this   |
|                    |                    |                    | value must be      |
|                    |                    |                    | consistent with    |
|                    |                    |                    | the required       |
|                    |                    |                    | infusion amount    |
|                    |                    |                    | and amounts added  |
|                    |                    |                    | in earlier steps   |
|                    |                    |                    | and is only        |
|                    |                    |                    | relevant as part   |
|                    |                    |                    | of a &lt;MASH&gt;  |
|                    |                    |                    | profile.<span      |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “1.5 qt/lb” or     |
|                    |                    |                    | “3.0 l/kg”         |
+--------------------+--------------------+--------------------+--------------------+
| **DECOCTION\_AMT** | No                 | Text               | Calculated volume  |
|                    |                    |                    | of mash to         |
|                    |                    |                    | decoct.<span       |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Only        |
|                    |                    |                    | applicable for a   |
|                    |                    |                    | decoction          |
|                    |                    |                    | step.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Includes    |
|                    |                    |                    | the units as in    |
|                    |                    |                    | “7.5 l” or “2.3    |
|                    |                    |                    | gal”               |
+--------------------+--------------------+--------------------+--------------------+
| **INFUSE\_TEMP**   | No                 | Text               | The calculated     |
|                    |                    |                    | infusion           |
|                    |                    |                    | temperature based  |
|                    |                    |                    | on the current     |
|                    |                    |                    | step, grain, and   |
|                    |                    |                    | other              |
|                    |                    |                    | settings.<span     |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Applicable  |
|                    |                    |                    | only for an        |
|                    |                    |                    | infusion           |
|                    |                    |                    | step.<span         |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>Includes    |
|                    |                    |                    | the units as in    |
|                    |                    |                    | “154 F” or “68 C”  |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_STEP\_T | No                 | Text               | Step temperature   |
| EMP**              |                    |                    | in user defined    |
|                    |                    |                    | temperature        |
|                    |                    |                    | units.<span        |
|                    |                    |                    | style="mso-spaceru |
|                    |                    |                    | n:yes">            |
|                    |                    |                    | </span>For example |
|                    |                    |                    | “154F” or “68 C”   |
+--------------------+--------------------+--------------------+--------------------+
| **DISPLAY\_INFUSE\ | No                 | Text               | Infusion amount    |
| _AMT**             |                    |                    | along with the     |
|                    |                    |                    | volume units as in |
|                    |                    |                    | “20 l” or “13 qt”  |
+--------------------+--------------------+--------------------+--------------------+

<span style="font-size:26.0pt"></span>

 

#### <span style="font-size:24.0pt;mso-bidi-font-size:12.0pt">Recipe Extensions</span>

The following may optionally be used in recipes.<span
style="mso-spacerun:yes">  </span>They must appear between the
&lt;RECIPE&gt; … &lt;/RECIPE&gt; tags.

<span style="font-size:26.0pt"></span>

 

**EST\_OG**

No

Text

Calculated estimate of the original gravity for this recipe along with
the units.

**EST\_FG**

No

Text

Calculated estimate for the final specific gravity of this recipe along
with the units as in “1.015 sg”

**EST\_COLOR**

No

Text

The estimated color of the beer in user defined color units.

**IBU**

No

IBUs

The estimated bitterness level of the beer in IBUs

**IBU\_METHOD**

No

List

May be “Rager”, “Tinseth” or “Garetz” corresponding to the
method/equation used to estimate IBUs for this recipe.

**EST\_ABV**

No

Percent

Estimated percent alcohol by volume for this recipe.

**ABV**

No

Percent

Actual alcohol by volume calculated from the OG and FG measured.

**ACTUAL\_EFFICIENCY**

No

Percent

The actual efficiency as calculated using the measured original and
final gravity.

**CALORIES**

No

Text

Calorie estimate based on the measured starting and ending gravity.<span
style="mso-spacerun:yes">  </span>Note that calories should be quoted in
“Cal” or kilocalories which is the normal dietary measure (i.e. a beer
is usually in the range of 100-250 calories per 12 oz).<span
style="mso-spacerun:yes">  </span>Examples “180 Cal/pint”,

**DISPLAY\_BATCH\_SIZE**

No

Text

Batch size in user defined units along with the units as in “5.0 gal”

**DISPLAY\_BOIL\_SIZE**

No

Text

Boil size with user defined units as in “6.3 gal”

**DISPLAY\_OG**

No

Text

Measured original gravity in user defined units as in “6.4 plato”

**DISPLAY\_FG**

No

Text

Measured final gravity in user defined units as in “1.035 sg”

**DISPLAY\_PRIMARY\_TEMP**

No

Text

Primary fermentation temperature in user defined units such as “64 F”

**DISPLAY\_SECONDARY\_TEMP**

No

Text

Secondary fermentation temperature in user defined units such as “56 F”

**DISPLAY\_TERTIARY\_TEMP**

No

Text

Tertiary temperature in user defined units such as “20 C”

**DISPLAY\_AGE\_TEMP**

No

Text

Temperature to use when aging the beer in user units such as “55 F”

**CARBONATION\_USED**

No

Text

Text description of the carbonation used such as “50g corn sugar” or
“Kegged at 20psi”

**DISPLAY\_CARB\_TEMP**

No

Text

Carbonation/Bottling temperature in appropriate units such as “40F” or
“32 C”

 

</div>
