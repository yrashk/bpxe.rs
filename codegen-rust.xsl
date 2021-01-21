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
        <xsl:choose>
            <xsl:when test="matches($string, '^t[A-Z][A-Za-z0-9_]+')">
                <xsl:value-of select="local:struct-case(substring($string, 2, string-length($string) - 1))"/>
            </xsl:when>
            <xsl:otherwise>
                <xsl:value-of select="
                    string-join(for $s in tokenize($string, '\W+')
                    return
                    concat(upper-case(substring($s, 1, 1)), substring($s, 2)), '')"/>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:function>
    
    <xsl:function name="local:type">
        <xsl:param name="type"/>
        <xsl:choose>
            <xsl:when test="$type= 'xsd:ID'">Id</xsl:when>
            <xsl:when test="$type = 'xsd:string'">String</xsl:when>
            <xsl:when test="$type = 'xsd:anyURI'">URI</xsl:when>
            <xsl:when test="$type = 'xsd:boolean'">bool</xsl:when>
            <xsl:when test="$type = 'xsd:integer'">Integer</xsl:when>
            <xsl:when test="$type = 'xsd:int'">Int</xsl:when>
            <xsl:otherwise>String</xsl:otherwise>
        </xsl:choose>
    </xsl:function>
    <xsl:function name="local:attributeType">
        <xsl:param name="node"/>
        <xsl:if test="$node/@use = 'optional' or not($node/@use)">
            <xsl:text>Option&lt;</xsl:text>
        </xsl:if>
        <xsl:value-of select="local:type($node/@type)"/>
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
    
    <xsl:function name="local:elementName">
        <xsl:param name="element"/>
        <xsl:value-of select="if ($element/@ref) then $element/@ref else $element/@name"/>
    </xsl:function>
    
    <xsl:function name="local:elementUnderscoreName">
        <xsl:param name="element"/>
        <xsl:choose> 
            <xsl:when test="xs:string($element/@maxOccurs) = 'unbounded'"><xsl:value-of select="local:pluralize(local:underscoreCase(local:elementName($element)))"/></xsl:when>
            <xsl:otherwise><xsl:value-of select="local:underscoreCase(local:elementName($element))"/></xsl:otherwise>
        </xsl:choose>
    </xsl:function>
    
    <xsl:function name="local:elementType">
        <xsl:param name="element"/>
        <xsl:variable name="name" select="local:elementName($element)"/>
        <xsl:variable name="subType" select="$element/@type"/>
        <xsl:variable name="subType" select="if (not($subType)) then $schema/xs:element[@name = $name]/@type else $subType"/>
        <xsl:variable name="subType" select="if (exists($schema/xs:complexType[@name = $subType])) then 
            local:struct-case($subType)
            else
            local:struct-case($name)
            "/>
        <xsl:variable name="subType" select="if ($subType = '') then local:type($subType) else $subType"/>
        <xsl:choose>
            <xsl:when test="$element/@minOccurs = 0 and (not($element/@maxOccurs) or $element/@maxOccurs = '1')"><xsl:text>Option&lt;</xsl:text></xsl:when>
            <xsl:when test="$element/@maxOccurs = 'unbounded'"><xsl:text>Vec&lt;</xsl:text></xsl:when>
        </xsl:choose>
        
        <xsl:value-of select="$subType"/>
        
        <xsl:choose>
            <xsl:when test="$element/@minOccurs = 0 and (not($element/@maxOccurs) or $element/@maxOccurs = '1')"><xsl:text>&gt;</xsl:text></xsl:when>
            <xsl:when test="$element/@maxOccurs = 'unbounded'"><xsl:text>&gt;</xsl:text></xsl:when>
        </xsl:choose>
    </xsl:function>
    
    <xsl:function name="local:attributes">
        <xsl:param name="type"/>
        <xsl:for-each select="$type//xs:attribute[@name]">
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
            use derive_more::{Deref, From};
            use std::fmt::Debug;
            use dyn_clone::DynClone;
            use tia::Tia;
        </xsl:text>
        
        <!-- Generate enum with all elements -->
        <xsl:text>#[derive(Debug, Clone, PartialEq)] pub enum Element {</xsl:text>
        <xsl:for-each-group select="$schema//xs:element[@name]" group-by="@name">
            <xsl:value-of select="local:struct-case(current-group()[1]/@name)"/>
            <xsl:text>,</xsl:text>
        </xsl:for-each-group>
        <xsl:text>}</xsl:text>
        
        <xsl:for-each select="$schema/xs:complexType[@name]">
            <xsl:call-template name="type">
                <xsl:with-param name="type" select="." />
            </xsl:call-template>
        </xsl:for-each>
        
        <xsl:for-each-group select="$schema//xs:element[@name and @type]" group-by="@name">
            <xsl:variable name="e" select="current-group()[1]"/>
            <xsl:if test="contains($e/@type, ':')">
                <xsl:variable name="name" select="$e/@name"/>
                <xsl:variable name="type" select="$e/@type"/>
                
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text>#[derive(Deref, From, Hash, Default, Clone, XmlRead, PartialEq, Debug)]#[from(forward)]#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub struct </xsl:text>
                <xsl:value-of select="local:struct-case($name)"/>
                <xsl:text xml:space="preserve"> {</xsl:text>
                <xsl:text>#[xml(text, cdata)]</xsl:text>
                <xsl:text>pub content:</xsl:text><xsl:value-of select="local:attributeType($type)"/>
                <xsl:text xml:space="preserve">}</xsl:text>
                
                <xsl:call-template name="documentElementTrait">
                    <xsl:with-param name="name" select="$name"></xsl:with-param>
                    <xsl:with-param name="typeName" select="$name"></xsl:with-param>
                    <xsl:with-param name="elements" select="()"></xsl:with-param>
                    <xsl:with-param name="id" select="false()"></xsl:with-param>
                    <xsl:with-param name="skipContainer" select="false()"/>
                </xsl:call-template>
                
            </xsl:if>
        </xsl:for-each-group>
    </xsl:template>
    
    
    <xsl:template name="type">
        <xsl:param name="type"/>
        <xsl:variable name="t" select="$type/@name"/>
        <xsl:variable name="typeName" select="$type/@name"/>
        <xsl:variable name="element" select="$schema/xs:element[@type = $t]"/>
        <xsl:variable name="name" select="$element/@name"/>
        
        <xsl:choose>
            <xsl:when test="$type/@abstract ">
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text >#[derive(Hash, XmlRead, Clone, PartialEq, Debug)]</xsl:text>
                <xsl:text>#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub enum </xsl:text>
                <xsl:value-of select="local:struct-case($typeName)"/>
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
                
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:text>impl From&lt;</xsl:text><xsl:value-of select="local:struct-case(./@name)"/><xsl:text xml:space="preserve">&gt; for </xsl:text>
                    <xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve"> { 
                        fn from(element: </xsl:text><xsl:value-of select="local:struct-case(./@name)"/><xsl:text>) -> Self {
                            Self::</xsl:text><xsl:value-of select="local:struct-case(./@name)"/><xsl:text>(element)
                        }
                    }    
                    </xsl:text>
                </xsl:for-each>
                
                <xsl:text xml:space="preserve">impl </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text> {</xsl:text>
                <xsl:text >pub fn into_inner(self) -> Box&lt;dyn DocumentElement&gt; { 
                    match self {
                </xsl:text>
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:value-of select="local:struct-case($typeName)"/><xsl:text>::</xsl:text><xsl:value-of select="local:struct-case(./@name)"/>
                    <xsl:text xml:space="preserve">(e) => Box::new(e) as Box&lt;dyn DocumentElement&gt;,</xsl:text>
                </xsl:for-each>
                <xsl:text>}}}</xsl:text>
                
                <xsl:text xml:space="preserve">
                    #[cast_to]
                    impl DocumentElementContainer for </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text> {
                        #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
                        fn find_by_id_mut(&amp;mut self, id: &amp;str) -> Option&lt;&amp;mut dyn DocumentElement&gt; {
                        match self {
                    </xsl:text>
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:value-of select="local:struct-case($name)"/><xsl:text>::</xsl:text><xsl:value-of select="local:struct-case(./@name)"/>(e) => e.find_by_id_mut(id),
                </xsl:for-each>
                _ => None,
                <xsl:text>
                    }
                    }
                    
                     #[allow(unreachable_patterns, clippy::match_single_binding, unused_variables)]
                        fn find_by_id(&amp;self, id: &amp;str) -> Option&lt;&amp;dyn DocumentElement&gt; {
                        match self {
                    </xsl:text>
                <xsl:for-each select="$elements[@substitutionGroup = $name]">
                    <xsl:value-of select="local:struct-case($name)"/><xsl:text>::</xsl:text><xsl:value-of select="local:struct-case(./@name)"/>(e) => e.find_by_id(id),
                </xsl:for-each>
                _ => None,
                <xsl:text>
                    }
                    }
                    }</xsl:text>
                  
                    <xsl:call-template name="documentElementTrait">
                    <xsl:with-param name="name" select="$name"></xsl:with-param>
                    <xsl:with-param name="typeName" select="$typeName"></xsl:with-param>
                    <xsl:with-param name="elements" select="()"></xsl:with-param>
                    <xsl:with-param name="id" select="false()"></xsl:with-param>
                    <xsl:with-param name="skipContainer" select="true()"/>
                    </xsl:call-template>
                
                <xsl:text xml:space="preserve">
                    /// Access to `</xsl:text><xsl:value-of select="$name"/><xsl:text>`
                        pub trait </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve">Type </xsl:text>
                <xsl:text>:</xsl:text>
                <xsl:if test="exists($type//xs:extension)">
                    <xsl:variable name="extTypeName" select="$type//xs:extension/@base"/>
                    <xsl:value-of select="local:struct-case($extTypeName)"/>
                    <xsl:text>Type + </xsl:text>
                </xsl:if>
                <xsl:text>Downcast + Debug + Send + DynClone {</xsl:text>
                <xsl:call-template name="traitFns">
                    <xsl:with-param name="type" select="$type"/>
                </xsl:call-template>
                <xsl:text>}</xsl:text>
                
                <xsl:text>dyn_clone::clone_trait_object!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type);</xsl:text>
                <xsl:text>impl_downcast!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type);</xsl:text>
                
                <xsl:text xml:space="preserve">
                    /// Mutable access to `</xsl:text><xsl:value-of select="$name"/><xsl:text>`
                        pub trait </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve">TypeMut </xsl:text>
                <xsl:text>:</xsl:text>
                <xsl:if test="exists($type//xs:extension)">
                    <xsl:variable name="extTypeName" select="$type//xs:extension/@base"/>
                    <xsl:value-of select="local:struct-case($extTypeName)"/>
                    <xsl:text>TypeMut + </xsl:text>
                </xsl:if>
                <xsl:text>Downcast + Debug + Send + DynClone + </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type {</xsl:text>
                <xsl:call-template name="mutTraitFns">
                    <xsl:with-param name="type" select="$type"/>
                </xsl:call-template>
                <xsl:text>}</xsl:text>
                
                <xsl:text>dyn_clone::clone_trait_object!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>TypeMut);</xsl:text>
                <xsl:text>impl_downcast!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>TypeMut);</xsl:text>                
            </xsl:when>
            <xsl:otherwise>
                <xsl:text xml:space="preserve">
                    /// Auto-generated from BPNM schema
                    ///
                    /// (See codegen-rust.xsl)
                </xsl:text>
                <xsl:text >#[derive(Tia, Hash, Default, Clone, XmlRead, PartialEq, Debug)]</xsl:text>
                <xsl:text>#[xml(tag = "bpmn:</xsl:text><xsl:value-of select="$name"/><xsl:text>")]</xsl:text>
                <xsl:text xml:space="preserve">pub struct </xsl:text>
                <xsl:value-of select="local:struct-case($typeName)"/>
                <xsl:text> {</xsl:text>
                
                
                
                <xsl:call-template name="content">
                    <xsl:with-param name="type" select="$type"></xsl:with-param>
                </xsl:call-template>
                
                <xsl:if test="$typeName = 'tExpression'">
                    // FIXME: This is a hack because obviously there's nothing about xsi:type
                    // in BPMN schema (and rightfully so)
                    #[tia(rg*="xsi_type",s)]
                    #[xml(attr = "xsi:type")]
                    pub xsi_type: Option&lt;String&gt;,
                    <xsl:text>#[tia("DocumentElementWithContent",rg*="content",
                    "DocumentElementWithContentMut",s,rmg*="content_mut")]</xsl:text>
                    <xsl:text>#[xml(text)]pub content: Option&lt;String&gt;,</xsl:text>
                </xsl:if>
                
                <xsl:text>}</xsl:text>
                
                <xsl:call-template name="documentElementTrait">
                    <xsl:with-param name="name" select="$name"></xsl:with-param>
                    <xsl:with-param name="typeName" select="$typeName"></xsl:with-param>
                    <xsl:with-param name="elements" select="local:elements($type)"></xsl:with-param>
                    <xsl:with-param name="id" select="local:hasId($type)"></xsl:with-param>
                    <xsl:with-param name="skipContainer" select="false()"/>
                </xsl:call-template>
                
                
                
                <xsl:text>
                    // Traits
                </xsl:text>
                <xsl:call-template name="traits">
                    <xsl:with-param name="type" select="$type"></xsl:with-param>
                    <xsl:with-param name="typeName" select="$typeName"></xsl:with-param>
                </xsl:call-template>
                
                <xsl:text>
                    //
                </xsl:text>
                
                
                <xsl:text xml:space="preserve">
                    /// Access to `</xsl:text><xsl:value-of select="$name"/><xsl:text>`
                        pub trait </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve">Type </xsl:text>
                <xsl:text>:</xsl:text>
                <xsl:if test="exists($type//xs:extension)">
                    <xsl:variable name="extTypeName" select="$type//xs:extension/@base"/>
                    <xsl:value-of select="local:struct-case($extTypeName)"/>
                    <xsl:text>Type +</xsl:text>
                </xsl:if>
                <xsl:if test="$type = 'tExtension'">
                    <xsl:text>DocumentElemenWithContent</xsl:text>
                </xsl:if>
                <xsl:text>Downcast + Debug + Send + DynClone {</xsl:text>
                <xsl:call-template name="traitFns">
                    <xsl:with-param name="type" select="$type"/>
                </xsl:call-template>
                <xsl:text>}</xsl:text>
                <xsl:text>dyn_clone::clone_trait_object!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type);</xsl:text>
                <xsl:text>impl_downcast!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type);</xsl:text>
                
                <xsl:text xml:space="preserve">
                    /// Mutable access to `</xsl:text><xsl:value-of select="$name"/><xsl:text>`
                        pub trait </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve">TypeMut </xsl:text>
                <xsl:text>:</xsl:text>
                <xsl:if test="exists($type//xs:extension)">
                    <xsl:variable name="extTypeName" select="$type//xs:extension/@base"/>
                    <xsl:value-of select="local:struct-case($extTypeName)"/>
                    <xsl:text>TypeMut +</xsl:text>
                </xsl:if>
                <xsl:if test="$type = 'tExtension'">
                    <xsl:text>DocumentElemenWithContent</xsl:text>
                </xsl:if>
                <xsl:text>Downcast + Debug + Send + DynClone + </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>Type  {</xsl:text>
                <xsl:call-template name="mutTraitFns">
                    <xsl:with-param name="type" select="$type"/>
                </xsl:call-template>
                <xsl:text>}</xsl:text>
                <xsl:text>dyn_clone::clone_trait_object!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>TypeMut);</xsl:text>
                <xsl:text>impl_downcast!(</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text>TypeMut);</xsl:text>
                
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>
    
    <xsl:template name="traits">
        <xsl:param name="type"></xsl:param>
        <xsl:param name="typeName"></xsl:param>
        <xsl:if test="exists($type//xs:extension)">
            <xsl:variable name="extTypeName" select="$type//xs:extension/@base"/>
            <xsl:variable name="extType" select="$schema/xs:complexType[@name=$extTypeName]"/>
            <xsl:if test="empty(local:attributes($extType)) and empty(local:elements($extType))">
                <xsl:text xml:space="preserve">#[cast_to] impl </xsl:text>
                <xsl:value-of select="local:struct-case($extTypeName)"/>
                <xsl:text xml:space="preserve">Type for </xsl:text><xsl:value-of select="local:struct-case($typeName)"/>
                <xsl:text>{}</xsl:text>
                <xsl:text xml:space="preserve">#[cast_to] impl </xsl:text>
                <xsl:value-of select="local:struct-case($extTypeName)"/>
                <xsl:text xml:space="preserve">TypeMut for </xsl:text><xsl:value-of select="local:struct-case($typeName)"/>
                <xsl:text>{}</xsl:text>
                <xsl:text>castable_to! {</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve"> => PartialEq&lt;</xsl:text>
                <xsl:value-of select="local:struct-case($typeName)"/>
                <xsl:text>&gt; }</xsl:text>
            </xsl:if>
            <xsl:text>castable_to! {</xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text xml:space="preserve"> => </xsl:text>
            <xsl:value-of select="local:struct-case($extTypeName)"/>
            <xsl:text xml:space="preserve">Type,</xsl:text>
            <xsl:value-of select="local:struct-case($extTypeName)"/>
            <xsl:text xml:space="preserve">TypeMut</xsl:text>
            <xsl:text>}</xsl:text>
            
            <xsl:call-template name="traits">
                <xsl:with-param name="type" select="$schema/xs:complexType[@name = $extTypeName]"></xsl:with-param>
                <xsl:with-param name="typeName" select="$typeName"></xsl:with-param>
            </xsl:call-template>
        </xsl:if>
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
            <xsl:text>#[tia("</xsl:text><xsl:value-of select="local:struct-case($type/@name)"/><xsl:text>Type",rg*="</xsl:text><xsl:value-of select="local:underscoreCase(@name)"/>
            <xsl:text>","</xsl:text><xsl:value-of select="local:struct-case($type/@name)"/><xsl:text>TypeMut",s)]</xsl:text>
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
            <xsl:variable name="name" select="local:elementName(.)"/>
            <xsl:choose>
                <xsl:when test="count($schema/xs:element[@substitutionGroup = $name]) > 1">
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
            <xsl:text>#[tia("</xsl:text><xsl:value-of select="local:struct-case($type/@name)"/><xsl:text>Type",rg*="</xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/>
            <xsl:text>","</xsl:text><xsl:value-of select="local:struct-case($type/@name)"/><xsl:text>TypeMut",s,rmg*="</xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/><xsl:text>_mut")]</xsl:text>
            <xsl:text xml:space="preserve">pub </xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/>
            <xsl:text>:</xsl:text>
            
            <xsl:value-of select="local:elementType(.)"/>
            
            
            <xsl:text>,</xsl:text>
        </xsl:for-each>
        
         
    </xsl:template>
    
    <xsl:template name="documentElementTrait">
        <xsl:param name="name" required="yes"/>
        <xsl:param name="typeName" required="yes"/>
        <xsl:param name="elements" required="yes"/>
        <xsl:param name="id" required="yes"/>
        <xsl:param name="skipContainer" required="yes" />
        <xsl:text xml:space="preserve">#[cast_to] impl DocumentElement for </xsl:text><xsl:value-of select="local:struct-case($typeName)"/><xsl:text> {
            fn element(&amp;self) -> Element {
            Element::</xsl:text><xsl:value-of select="local:struct-case($name)"/><xsl:text>
                }
                }</xsl:text>
        <xsl:if test="not($skipContainer)">
            <xsl:call-template name="documentElementContainerTrait">
                <xsl:with-param name="name" select="$name"></xsl:with-param>
                <xsl:with-param name="typeName" select="$typeName"></xsl:with-param>
                <xsl:with-param name="elements" select="$elements"></xsl:with-param>
                <xsl:with-param name="id" select="$id"></xsl:with-param>
            </xsl:call-template>
        </xsl:if>
    </xsl:template>
    
    <xsl:template name="documentElementContainerTrait">
        <xsl:param name="name" required="yes"/>
        <xsl:param name="typeName" required="yes"/>
        <xsl:param name="elements" required="yes"/>
        <xsl:param name="id" required="yes"/>
        <xsl:text xml:space="preserve">#[allow(unused_variables)]#[cast_to] impl DocumentElementContainer for </xsl:text><xsl:value-of select="local:struct-case($typeName)"/>
        <xsl:choose>
            <!-- use default implementation -->
            <xsl:when test="empty($elements) and not($id)">{}</xsl:when>
            <xsl:otherwise>
                
                <xsl:text> {
                    fn find_by_id_mut(&amp;mut self, id: &amp;str) -> Option&lt;&amp;mut dyn DocumentElement&gt; {
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
                    <xsl:text>.find_by_id_mut(id) {
                        return Some(e);
                        }</xsl:text>
                </xsl:for-each>
                <xsl:text>
                    None
                    }
                </xsl:text>
                
                <xsl:text> 
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
                    None
                    }
                </xsl:text>
                
                
                <xsl:text>}</xsl:text>
            </xsl:otherwise>
        </xsl:choose>
        
    </xsl:template>
    
    <xsl:template name="traitFns">
        <xsl:param name="type"/>
        <xsl:for-each select="local:attributes($type)">
            <xsl:text>/// Get value of attribute `</xsl:text><xsl:value-of select="./@name"/><xsl:text xml:space="preserve">`
                fn </xsl:text><xsl:value-of select="local:underscoreCase(./@name)"/><xsl:text>(&amp; self) -> &amp;</xsl:text>
            <xsl:value-of select="local:attributeType(.)"/><xsl:text>;</xsl:text>
        </xsl:for-each>
        <xsl:for-each select="local:elements($type)">
            <xsl:text>
                /// Get value of `</xsl:text><xsl:value-of select="local:elementName(.)"/><xsl:text xml:space="preserve">` child
                    fn </xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/><xsl:text>(&amp; self) -> &amp;</xsl:text>
            <xsl:value-of select="local:elementType(.)"/><xsl:text>;</xsl:text>
        </xsl:for-each>
    </xsl:template>
    <xsl:template name="mutTraitFns">
        <xsl:param name="type"/>
        <xsl:for-each select="local:attributes($type)">
            
            <xsl:text>/// Set value of attribute `</xsl:text><xsl:value-of select="./@name"/><xsl:text xml:space="preserve">`
                fn set_</xsl:text><xsl:value-of select="local:underscoreCase(./@name)"/><xsl:text>(&amp;mut self, value: </xsl:text><xsl:value-of select="local:attributeType(.)"/><xsl:text>);</xsl:text>
        </xsl:for-each>
        <xsl:for-each select="local:elements($type)">
            
            <xsl:text>
                /// Get a mutable value of `</xsl:text><xsl:value-of select="local:elementName(.)"/><xsl:text xml:space="preserve">` child
                    fn </xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/><xsl:text xml:space="preserve">_mut(&amp;mut self) -> &amp;mut </xsl:text>
            <xsl:value-of select="local:elementType(.)"/><xsl:text>;</xsl:text>
            <xsl:text>
                /// Set value of `</xsl:text><xsl:value-of select="local:elementName(.)"/><xsl:text xml:space="preserve">` child
                    fn set_</xsl:text><xsl:value-of select="local:elementUnderscoreName(.)"/><xsl:text>(&amp;mut self, value: </xsl:text><xsl:value-of select="local:elementType(.)"/><xsl:text>);</xsl:text>
            
        </xsl:for-each>
    </xsl:template>
    
    
    
</xsl:stylesheet>
