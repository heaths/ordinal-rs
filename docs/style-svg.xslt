<xsl:stylesheet version="1.0"
  xmlns:str="http://exslt.org/strings"
  xmlns:svg="http://www.w3.org/2000/svg"
  xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:output method="xml" version="1.0" encoding="utf-8" cdata-section-elements="svg:style"/>
  <xsl:template match="svg:svg">
    <xsl:element name="svg" namespace="http://www.w3.org/2000/svg">
      <xsl:apply-templates select="@*"/>

      <!-- Use @media queries to stylize text for readability in different themes -->
      <xsl:element name="style" namespace="http://www.w3.org/2000/svg">
        <xsl:attribute name="style">text/css</xsl:attribute>
        <xsl:text disable-output-escaping="yes"><![CDATA[
text { fill: #000000 }
polygon { fill: #1F78B4 }
polyline { fill: none; stroke: #000000 }
text.self { fill: #00cf00 }
polygon.self { fill: #00cf00 }
@media (prefers-color-scheme: dark) {
  text { fill: #ffffff }
  polygon { fill: #1F78B4 }
  polyline { fill: none; stroke: #ffffff }
  text.self { fill: #00cf00 }
  polygon.self { fill: #00cf00 }
}
]]></xsl:text>
      </xsl:element>
      <xsl:apply-templates select="node()"/>
    </xsl:element>
  </xsl:template>

  <!-- Use a different color for our own text and plot polyline nodes. -->
  <xsl:template match="svg:text">
    <xsl:copy>
      <xsl:if test="str:tokenize(text(), '/')[2] = 'ordinal-trait'">
        <xsl:attribute name="class">self</xsl:attribute>
      </xsl:if>
      <xsl:apply-templates select="node() | @*"/>
    </xsl:copy>
  </xsl:template>

  <!-- Get Y axis values of our own svg:text nodes. -->
  <xsl:key name="yaxes" match="/svg:svg/svg:text[str:tokenize(text(), '/')[2] = 'ordinal-trait']" use="number(@y)"/>
  <xsl:template match="svg:polygon">
    <xsl:copy>
      <xsl:variable name="y" select="number(str:tokenize(str:tokenize(@points, ' ')[1], ',')[2])"/>
      <!-- Generate reports' y axis might be off by one. -->
      <xsl:if test="key('yaxes', $y) | key('yaxes', $y - 1) | key('yaxes', $y + 1)">
        <xsl:attribute name="class">self</xsl:attribute>
      </xsl:if>
      <xsl:apply-templates select="node() | @*"/>
    </xsl:copy>
  </xsl:template>

  <!-- Remove the "Input" label that overlaps Y axis labels -->
  <xsl:template match="svg:text[normalize-space() = 'Input']"/>

  <!-- Remove hardcoded colors I want to vary on @media queries -->
  <xsl:template match="svg:text[@fill = '#000000']/@fill"/>
  <xsl:template match="svg:polyline[@stroke = '#000000']/@stroke"/>
  <xsl:template match="svg:polygon/@fill"/>

  <!-- Split templates because of https://github.com/ballsteve/xrust/issues/95 -->
  <xsl:template match="node() | @*">
    <xsl:copy>
      <xsl:apply-templates select="node() | @*"/>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
