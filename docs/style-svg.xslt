<xsl:stylesheet version="1.0"
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
polyline { fill: none; stroke: #000000 }
@media (prefers-color-scheme: dark) {
  text { fill: #ffffff }
  polyline { fill: none; stroke: #ffffff }
}
]]></xsl:text>
      </xsl:element>

      <xsl:apply-templates select="node()"/>
    </xsl:element>
  </xsl:template>

  <!-- Remove the "Input" label that overlaps Y axis labels -->
  <xsl:template match="svg:text[normalize-space() = 'Input']"/>

  <!-- Remove hardcoded colors I want to vary on @media queries -->
  <xsl:template match="svg:text[@fill = '#000000']/@fill"/>
  <xsl:template match="svg:polyline[@stroke = '#000000']/@stroke"/>

  <!-- Split templates because of https://github.com/ballsteve/xrust/issues/95 -->
  <xsl:template match="node() | @*">
    <xsl:copy>
      <xsl:apply-templates select="node() | @*"/>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
