import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper

def jsonSlurper = new JsonSlurper()

response = WS.sendRequest(findTestObject('Posts/Edit Post By Id'))

if (WS.verifyResponseStatusCode(response, 200, FailureHandling.OPTIONAL)) {
	WS.verifyElementPropertyValue(response, 'name', name)

	WS.verifyElementPropertyValue(response, 'email', email)

	WS.verifyElementPropertyValue(response, 'status', status)

	WS.verifyElementPropertyValue(response, 'gender', gender)

	def JsonResponse = jsonSlurper.parseText(response.getResponseText())

	GlobalVariable.userId = JsonResponse.id
} else if (WS.verifyResponseStatusCode(response, 401, FailureHandling.OPTIONAL)) {
	System.out.println('Authentication failed')
} else if (WS.verifyResponseStatusCode(response, 422, FailureHandling.STOP_ON_FAILURE)) {
	System.out.println('Data validation failed (in response to a POST request, for example). Please check the response body for detailed error messages.')

	System.out.println('field: ' + jsonSlurper.parseText(response.getResponseText())[0].field)

	System.out.println('message: ' + jsonSlurper.parseText(response.getResponseText())[0].message)
} else if (WS.verifyResponseStatusCode(response, 404, FailureHandling.STOP_ON_FAILURE) {
	System.out.println('Resource Post with id' + Global + 'Not Found')
}



