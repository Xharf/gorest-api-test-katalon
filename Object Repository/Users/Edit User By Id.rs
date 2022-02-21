<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Edit User By Id</name>
   <tag></tag>
   <elementGuidId>ebda30a3-5f5e-4d4c-82ff-fa5243d58bb2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;email\&quot;:\&quot;${email}\&quot;,\n    \&quot;gender\&quot;:\&quot;${gender}\&quot;,\n    \&quot;status\&quot;:\&quot;${status}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${GlobalVariable.token}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://gorest.co.in/public/v2/users/${userId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'${GlobalVariable.userId}'</defaultValue>
      <description></description>
      <id>65b67211-4f95-438b-86f3-be7403cdbe01</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name</defaultValue>
      <description></description>
      <id>c0089bc7-1f53-455b-88eb-c1596cc59f7b</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'female'</defaultValue>
      <description></description>
      <id>350dc973-dc83-4d01-ba79-3a0673dd85cf</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>'active'</defaultValue>
      <description></description>
      <id>f13964e7-3b1c-4f47-8634-cdd40d223525</id>
      <masked>false</masked>
      <name>status</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>a3c472c5-bcf2-4778-b343-7fc10a39248a</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
