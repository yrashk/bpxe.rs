<?xml version="1.0" encoding="UTF-8"?>
<!-- To regenerate Rust code using this stylesheet, use ./codegen.sh -->
<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
    xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:local="http://bpxe.org/"
    xmlns:map="http://www.w3.org/2005/xpath-functions/map"
    exclude-result-prefixes="xs" version="3.0">
    
    <xsl:output method="text"/>
    
    <xsl:variable name="schema"
        select="(/ | document(/xs:schema/xs:include/@schemaLocation))/xs:schema"/>
    <xsl:variable name="elements" select="$schema//xs:element"/>
    
    <xsl:accumulator name="knownElements" initial-value="map{}">
        <xsl:accumulator-rule match="xs:element[@name]" select="map:put($value, string(./@name), true())" phase="end"/>
    </xsl:accumulator>
    
    <xsl:mode use-accumulators="#all"/>
    
    <xsl:function name="local:underscoreCase">
        <xsl:param name="string"/>
        <xsl:choose>
            <xsl:when test="$string = 'type'">typ</xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="lower-case(replace($string, '([a-z])([A-Z][a-z])', '$1_$2'))"/>
            </xsl:otherwise>
        </xsl:choose>
        
    </xsl:function>
    <xsl:function name="local:pluralize">
        <xsl:param name="word"/>
        <xsl:choose>
            <xsl:when test="ends-with($word, 'ss') or ends-with($word, 'x') or ends-with($word, 'ch') or ends-with($word, 'sh')"><xsl:value-of select="concat($word, 'es')"/></xsl:when>
            <xsl:when test="ends-with($word, 'ey') or ends-with($word, 'ay') or ends-with($word, 'oy')"><xsl:value-of select="concat($word, 's')"/></xsl:when>
            <xsl:when test="ends-with($word, 'y')"><xsl:value-of select="concat(substring($word,0, string-length($word) - 1), 'ies')"/></xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="concat($word, 's')"/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:function>
    
    <xsl:function name="local:struct-case">
        <xsl:param name="string"/>
        <xsl:value-of select="
            string-join(for $s in tokenize($string, '\W+')
            return
            concat(upper-case(substring($s, 1, 1)), substring($s, 2)), '')"/>
    </xsl:function>
    
    <xsl:function name="local:attributeType">
        <xsl:param name="node"/>
        <xsl:if test="$node/@use = 'optional' or not($node/@use)">
            <xsl:text>Option&lt;</xsl:text>
        </xsl:if>
        <xsl:choose>
            <xsl:when test="$node/@type = 'xsd:ID'">Id</xsl:when>
            <xsl:when test="$node/@type = 'xsd:string'">String</xsl:when>
            <xsl:when test="$node/@type = 'xsd:anyURI'">URI</xsl:when>
            <xsl:when test="$node/@type = 'xsd:boolean'">bool</xsl:when>
            <xsl:when test="$node/@type = 'xsd:integer'">Integer</xsl:when>
            <xsl:when test="$node/@type = 'xsd:int'">Int</xsl:when>
            <xsl:otherwise>String</xsl:otherwise>
        </xsl:choose>
        <xsl:if test="$node/@use = 'optional' or not($node/@use)">
            <xsl:text>&gt;</xsl:text>
        </xsl:if>
    </xsl:function>
    
    <xsl:function name="local:elements">
        <xsl:param name="type"/>
        <xsl:variable name="subelements"
            select="$type//xs:element[@ref and not(contains(@ref, ':'))] | $type//xs:element[@name]"/>
        <xsl:for-each select="$subelements">
            <xsl:sequence select="."/>
        </xsl:for-each>
    </xsl:function>
    
    <xsl:function name="local:hasId">
        <xsl:param name="type"/>
        <xsl:choose>
            <xsl:when test="exists($type//xs:attribute[@name='id'])">
                <xsl:value-of select="true()"/>
            </xsl:when>
            <xsl:otherwise>
        <xsl:for-each select="$type//xs:extension">
            <xsl:variable name="extTypeName" select="./@base"/>
            <xsl:value-of select="local:hasId($schema/xs:complexType[@name = $extTypeName])"/>
        </xsl:for-each>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:function>
    
    <xsl:strip-space elements="*"/>
    
    <xsl:template match="/">
        
        <xsl:text>
            // This file is generated from BPMN 2.0 schema using `codegen.sh` script
            use strong_xml::XmlRead;
        </xsl:text>
        
        <!-- Generate enum with all elements -->
        <xsl:text>#[derive(Debug, Clone, PartialEq)] pub enum Element {</xsl:text>
        <xsl:for-each-group select="$schema//xs:element[@name]" group-by="@name">
            <xsl:value-of select="local:struct-case(./@name)"/>
            <xsl:text>,</xsl:text>
        </xsl:for-each-group>
        <xsl:text>}</xsl:text>
        
        <xsl:for-each-group select="$schema//xs:element[@name]" group-by="@name">
            <xsl:call-template name="element">
                <xsl:with-param name="element" select="."/>
            </xsl:call-template>
        </xsl:for-each-group>
    </xsl:template>
    
    
    <xsl:template name="element">
        <xsl:param name="element"/>
        
        
        <xsl:variable name="name" select="$element/@name"/>
        <xsl:variable name="elementType" select="$element/@type"/>
        <xsl:variable name="type" select="$schema/xs:complexType[@name = $elementType]"/>
        
        <xsl:choose>
            <xsl:when test="not($type) and local:attributeType($elementType)">
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text>#[derive(Default, Clone, XmlRead, PartialEq, Debug)]#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub struct </xsl:text>
                <xsl:value-of select="local:struct-case($name)"/>
                <xsl:text xml:space="preserve"> {</xsl:text>
                <xsl:text>#[xml(text, cdata)]</xsl:text>
                <xsl:text>pub content:</xsl:text><xsl:value-of select="local:attributeType($elementType)"/>
                <xsl:text xml:space="preserve">}</xsl:text>
                
                <xsl:call-template name="documentElementTrait">
                    <xsl:with-param name="name" select="$name"></xsl:with-param>
                    <xsl:with-param name="elements" select="()"></xsl:with-param>
                    <xsl:with-param name="id" select="false()"></xsl:with-param>
                </xsl:call-template>
                
            </xsl:when>
            <xsl:when test="$type/@abstract and count($schema//xs:element[@substitutionGroup = $name]) > 0">
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text >#[derive(XmlRead, Clone, PartialEq, Debug)]</xsl:text>
                <xsl:text>#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub enum </xsl:text>
                <xsl:value-of select="local:struct-case($name)"/>
                <xsl:text>{</xsl:text>
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:text>#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="./@name"/><xsl:text>")]</xsl:text>
                    <xsl:value-of select="local:struct-case(./@name)"/>
                    <xsl:text>(</xsl:text>
                    <xsl:value-of select="local:struct-case(./@name)"/>
                    <xsl:text></xsl:text>
                    <xsl:text>),</xsl:text>
                </xsl:for-each>
                
                <xsl:text>}</xsl:text>
                
                <xsl:text xml:space="preserve">
                    impl DocumentElementContainer for </xsl:text><xsl:value-of select="local:struct-case($name)"/><xsl:text> {
                        fn find_by_id(&amp;self, id: &amp;str) -> Option&lt;&amp;dyn DocumentElement&gt; {
                        match self {
                    </xsl:text>
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:value-of select="local:struct-case($name)"/><xsl:text>::</xsl:text><xsl:value-of select="local:struct-case(./@name)"/>(e) => e.find_by_id(id),
                </xsl:for-each>
                <xsl:text>
                    }
                    }
                    }</xsl:text>
                
            </xsl:when>
            <xsl:otherwise>
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text >#[derive(Default, Clone, XmlRead, PartialEq, Debug)]</xsl:text>
                <xsl:text>#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub struct </xsl:text>
                <xsl:value-of select="local:struct-case($name)"/>
                <xsl:text> {</xsl:text>
                
                
                
                <xsl:call-template name="content">
                    <xsl:with-param name="type" select="$type"></xsl:with-param>
                </xsl:call-template>
                
                
                <xsl:text>}</xsl:text>
                
                <xsl:call-template name="documentElementTrait">
                    <xsl:with-param name="name" select="$name"></xsl:with-param>
                    <xsl:with-param name="elements" select="local:elements($type)"></xsl:with-param>
                    <xsl:with-param name="id" select="local:hasId($type)"></xsl:with-param>
                </xsl:call-template>
                
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>
    
    <xsl:template name="content">
        <xsl:param name="type"></xsl:param>
        
        <!-- Inherited -->
        
        <xsl:for-each select="$type//xs:extension">
            <xsl:variable name="extTypeName" select="./@base"/>
            
            <xsl:call-template name="content">
                <xsl:with-param name="type" select="$schema/xs:complexType[@name = $extTypeName]"></xsl:with-param>
            </xsl:call-template>
            
            
        </xsl:for-each>
        
        <!-- Attributes -->
        <xsl:for-each select="$type//xs:attribute">
            <xsl:text>#[xml(attr = "</xsl:text><xsl:value-of select="@name"/><xsl:text>")]</xsl:text>
            <xsl:text xml:space="preserve">pub </xsl:text>
            <xsl:value-of select="local:underscoreCase(@name)"/>
            <xsl:text>:</xsl:text>
            <xsl:value-of select="local:attributeType(.)"/>
            <xsl:text>,</xsl:text>
        </xsl:for-each>
        
        
        <!-- Children -->
        <xsl:variable name="subelements"
            select="$type//xs:element[@ref and not(contains(@ref, ':'))] | $type//xs:element[@name]"/>
        <xsl:for-each select="$subelements">
            <xsl:variable name="name" select="if (./@ref) then ./@ref else ./@name"/>
            <xsl:variable name="subType" select="if (./@ref) then $schema/xs:element[@name=$name]/@type else ./@type"/>
            <xsl:choose>
                <xsl:when test="$schema/xs:complexType[@name = $subType]/@abstract and count($schema/xs:element[@substitutionGroup = $name]) > 1">
                    <xsl:text>#[xml(</xsl:text>
                    <xsl:for-each select="$schema/xs:element[@substitutionGroup = $name]">
                        <xsl:text>child = "bpmn:</xsl:text><xsl:value-of select="./@name"/><xsl:text>",</xsl:text>
                    </xsl:for-each>
                    <xsl:text>)]</xsl:text>
                </xsl:when>
                <xsl:otherwise>
                    <xsl:text>#[xml(child = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                </xsl:otherwise>
            </xsl:choose>
            <xsl:text xml:space="preserve">pub </xsl:text>
            <xsl:choose> 
                <xsl:when test="xs:string(./@maxOccurs) = 'unbounded'"><xsl:value-of select="local:pluralize(local:underscoreCase($name))"/></xsl:when>
                <xsl:otherwise><xsl:value-of select="local:underscoreCase($name)"/></xsl:otherwise>
            </xsl:choose>
            <xsl:text>:</xsl:text>
            
            <xsl:choose>
                <xsl:when test="./@minOccurs = 0 and (not(./@maxOccurs) or ./@maxOccurs = '1')">Option&lt;</xsl:when>
                <xsl:when test="./@maxOccurs = 'unbounded'">Vec&lt;</xsl:when>
            </xsl:choose>
            
            <xsl:value-of select="local:struct-case($name)"/>
            <xsl:text></xsl:text>
            
            
            <xsl:choose>
                <xsl:when test="./@minOccurs = 0 and (not(./@maxOccurs) or ./@maxOccurs = '1')">&gt;</xsl:when>
                <xsl:when test="./@maxOccurs = 'unbounded'">&gt;</xsl:when>
            </xsl:choose>
            
            
            
            <xsl:text>,</xsl:text>
        </xsl:for-each>
        
        <!-- Any -->
        <xsl:if test="$type//xs:any">
            <xsl:text>
                #[xml(text, cdata)]
                content: String,</xsl:text>
        </xsl:if>
        
    </xsl:template>
    
    <xsl:template name="documentElementTrait">
        <xsl:param name="name" required="yes"/>
        <xsl:param name="elements" required="yes"/>
        <xsl:param name="id" required="yes"/>
        <xsl:text xml:space="preserve">impl DocumentElement for </xsl:text><xsl:value-of select="local:struct-case($name)"/><xsl:text> {
            fn element(&amp;self) -> Element {
            Element::</xsl:text><xsl:value-of select="local:struct-case($name)"/><xsl:text>
                }
                }</xsl:text>
        <xsl:call-template name="documentElementContainerTrait">
            <xsl:with-param name="name" select="$name"></xsl:with-param>
            <xsl:with-param name="elements" select="$elements"></xsl:with-param>
            <xsl:with-param name="id" select="$id"></xsl:with-param>
        </xsl:call-template>
    </xsl:template>
    
    <xsl:template name="documentElementContainerTrait">
        <xsl:param name="name" required="yes"/>
        <xsl:param name="elements" required="yes"/>
        <xsl:param name="id" required="yes"/>
        <xsl:text xml:space="preserve">#[allow(unused_variables)] impl DocumentElementContainer for </xsl:text><xsl:value-of select="local:struct-case($name)"/><xsl:text> {
            fn find_by_id(&amp;self, id: &amp;str) -> Option&lt;&amp;dyn DocumentElement&gt; {
        </xsl:text>
        
        <xsl:if test="$id = true()">
            <xsl:text>
                if let Some(ref id_) = self.id {
                if id_ == id {
                return Some(self);
                }
                }
            </xsl:text>
        </xsl:if>
        
        <xsl:for-each select="$elements">
            <xsl:variable name="name" select="if (./@ref) then ./@ref else ./@name"/>
            
            <xsl:text> if let Some(e) = self.</xsl:text>
            <xsl:choose> 
                <xsl:when test="xs:string(./@maxOccurs) = 'unbounded'"><xsl:value-of select="local:pluralize(local:underscoreCase($name))"/></xsl:when>
                <xsl:otherwise><xsl:value-of select="local:underscoreCase($name)"/></xsl:otherwise>
            </xsl:choose>
            <xsl:text>.find_by_id(id) {
                return Some(e);
                }</xsl:text>
        </xsl:for-each>
        <xsl:text>
            return None;
            }
            }</xsl:text>
    </xsl:template>
    
</xsl:stylesheet>
