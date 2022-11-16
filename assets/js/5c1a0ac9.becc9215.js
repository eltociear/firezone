"use strict";(self.webpackChunkfirezone_docs=self.webpackChunkfirezone_docs||[]).push([[4886],{3905:(t,e,n)=>{n.d(e,{Zo:()=>d,kt:()=>c});var a=n(67294);function r(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function o(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(t);e&&(a=a.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,a)}return n}function l(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?o(Object(n),!0).forEach((function(e){r(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function i(t,e){if(null==t)return{};var n,a,r=function(t,e){if(null==t)return{};var n,a,r={},o=Object.keys(t);for(a=0;a<o.length;a++)n=o[a],e.indexOf(n)>=0||(r[n]=t[n]);return r}(t,e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(t);for(a=0;a<o.length;a++)n=o[a],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(r[n]=t[n])}return r}var u=a.createContext({}),p=function(t){var e=a.useContext(u),n=e;return t&&(n="function"==typeof t?t(e):l(l({},e),t)),n},d=function(t){var e=p(t.components);return a.createElement(u.Provider,{value:e},t.children)},m={inlineCode:"code",wrapper:function(t){var e=t.children;return a.createElement(a.Fragment,{},e)}},s=a.forwardRef((function(t,e){var n=t.components,r=t.mdxType,o=t.originalType,u=t.parentName,d=i(t,["components","mdxType","originalType","parentName"]),s=p(n),c=r,g=s["".concat(u,".").concat(c)]||s[c]||m[c]||o;return n?a.createElement(g,l(l({ref:e},d),{},{components:n})):a.createElement(g,l({ref:e},d))}));function c(t,e){var n=arguments,r=e&&e.mdxType;if("string"==typeof t||r){var o=n.length,l=new Array(o);l[0]=s;var i={};for(var u in e)hasOwnProperty.call(e,u)&&(i[u]=e[u]);i.originalType=t,i.mdxType="string"==typeof t?t:r,l[1]=i;for(var p=2;p<o;p++)l[p]=n[p];return a.createElement.apply(null,l)}return a.createElement.apply(null,n)}s.displayName="MDXCreateElement"},13465:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>u,contentTitle:()=>l,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>p});var a=n(87462),r=(n(67294),n(3905));const o={title:"Jumpcloud",sidebar_position:2,description:"Firezone's WireGuard\xae-based remote access platform supports using Jumpcloud as a single sign-on provider using a SAML 2.0 connector."},l=void 0,i={unversionedId:"authenticate/saml/jumpcloud",id:"authenticate/saml/jumpcloud",title:"Jumpcloud",description:"Firezone's WireGuard\xae-based remote access platform supports using Jumpcloud as a single sign-on provider using a SAML 2.0 connector.",source:"@site/docs/authenticate/saml/jumpcloud.mdx",sourceDirName:"authenticate/saml",slug:"/authenticate/saml/jumpcloud",permalink:"/authenticate/saml/jumpcloud",draft:!1,editUrl:"https://github.com/firezone/firezone/tree/master/docs/docs/authenticate/saml/jumpcloud.mdx",tags:[],version:"current",sidebarPosition:2,frontMatter:{title:"Jumpcloud",sidebar_position:2,description:"Firezone's WireGuard\xae-based remote access platform supports using Jumpcloud as a single sign-on provider using a SAML 2.0 connector."},sidebar:"tutorialSidebar",previous:{title:"Okta",permalink:"/authenticate/saml/okta"},next:{title:"OneLogin",permalink:"/authenticate/saml/onelogin"}},u={},p=[{value:"Create a SAML connector",id:"create-a-saml-connector",level:2},{value:"Add SAML identity provider to Firezone",id:"add-saml-identity-provider-to-firezone",level:2}],d={toc:p};function m(t){let{components:e,...n}=t;return(0,r.kt)("wrapper",(0,a.Z)({},d,n,{components:e,mdxType:"MDXLayout"}),(0,r.kt)("admonition",{type:"note"},(0,r.kt)("p",{parentName:"admonition"},"This guide assumes you have completed the prerequisite steps outlined ",(0,r.kt)("a",{parentName:"p",href:"/authenticate/saml"},"here"),".")),(0,r.kt)("p",null,"Firezone supports Single Sign-On (SSO) using Jumpcloud through the generic SAML 2.0 connector.\nThis guide will walk you through how to configure the integration."),(0,r.kt)("h2",{id:"create-a-saml-connector"},"Create a SAML connector"),(0,r.kt)("p",null,"In the Jumpcloud admin portal, create a new App under\nthe SSO tab. At the bottom of the popup window, click ",(0,r.kt)("inlineCode",{parentName:"p"},"Custom SAML App"),"."),(0,r.kt)("p",null,"After entering your desired value for ",(0,r.kt)("inlineCode",{parentName:"p"},"Display Label"),", click the ",(0,r.kt)("inlineCode",{parentName:"p"},"SSO")," tab,\nthen use the following configuration values:"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},"Setting"),(0,r.kt)("th",{parentName:"tr",align:null},"Value"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"IdP Entity ID"),(0,r.kt)("td",{parentName:"tr",align:null},"Any unique string will work, e.g. ",(0,r.kt)("inlineCode",{parentName:"td"},"firezone-jumpcloud"),".")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"SP Entity ID"),(0,r.kt)("td",{parentName:"tr",align:null},"This should be the same as your Firezone ",(0,r.kt)("inlineCode",{parentName:"td"},"SAML_ENTITY_ID"),", defaults to ",(0,r.kt)("inlineCode",{parentName:"td"},"urn:firezone.dev:firezone-app"),".")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"ACS URL"),(0,r.kt)("td",{parentName:"tr",align:null},"This is your Firezone ",(0,r.kt)("inlineCode",{parentName:"td"},"EXTERNAL_URL/auth/saml/sp/consume/:config_id"),", e.g. ",(0,r.kt)("inlineCode",{parentName:"td"},"https://firezone.company.com/auth/saml/sp/consume/jumpcloud"),".")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"SAMLSubject NameID"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"email"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"SAMLSubject NameID Format"),(0,r.kt)("td",{parentName:"tr",align:null},"Leave at the default.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Signature Algorithm"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"RSA-SHA256"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Sign Assertion"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Checked"),".")))),(0,r.kt)("p",null,"Leave the rest of the settings unchanged, then click the ",(0,r.kt)("inlineCode",{parentName:"p"},"activate")," button at the bottom-right."),(0,r.kt)("p",null,"Your Jumpcloud configuration should now resemble the following:"),(0,r.kt)("p",null,(0,r.kt)("img",{parentName:"p",src:"https://user-images.githubusercontent.com/167144/202085625-41a818d4-bc9d-4f77-b2db-43656fa42804.png",alt:"Jumpcloud SAML"})),(0,r.kt)("p",null,"Now, download the IdP Metadata document by selecting the App you just created\nand then clicking the ",(0,r.kt)("inlineCode",{parentName:"p"},"export metadata")," button in the upper-right. You'll need\nto copy-paste the contents of this document into the Firezone portal in the next step."),(0,r.kt)("h2",{id:"add-saml-identity-provider-to-firezone"},"Add SAML identity provider to Firezone"),(0,r.kt)("p",null,"In the Firezone portal, add a SAML identity provider under the Security tab\nby filling out the following information:"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},"Setting"),(0,r.kt)("th",{parentName:"tr",align:null},"Value"),(0,r.kt)("th",{parentName:"tr",align:null},"Notes"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Config ID"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"jumpcloud")),(0,r.kt)("td",{parentName:"tr",align:null},"Firezone uses this value to construct endpoints required in the SAML authentication flow (e.g., receiving assertions, login requests).")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Label"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"Jumpcloud")),(0,r.kt)("td",{parentName:"tr",align:null},"Appears on the sign in button for authentication.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Base URL"),(0,r.kt)("td",{parentName:"tr",align:null},"Leave unchanged."),(0,r.kt)("td",{parentName:"tr",align:null})),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Metadata"),(0,r.kt)("td",{parentName:"tr",align:null},"see note"),(0,r.kt)("td",{parentName:"tr",align:null},"Copy-paste the contents of the SAML metadata document you downloaded in the previous step from Jumpcloud.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Sign assertions"),(0,r.kt)("td",{parentName:"tr",align:null},"Checked."),(0,r.kt)("td",{parentName:"tr",align:null})),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Sign metadata"),(0,r.kt)("td",{parentName:"tr",align:null},"Checked."),(0,r.kt)("td",{parentName:"tr",align:null})),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Require signed assertions"),(0,r.kt)("td",{parentName:"tr",align:null},"Checked."),(0,r.kt)("td",{parentName:"tr",align:null})),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},"Required signed envelopes"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Unchecked.")),(0,r.kt)("td",{parentName:"tr",align:null})))),(0,r.kt)("p",null,"Your Firezone configuration should now resemble the following:"),(0,r.kt)("p",null,(0,r.kt)("img",{parentName:"p",src:"https://user-images.githubusercontent.com/167144/202086477-50927200-4315-4c65-865a-33da6157af1b.png",alt:"Firezone SAML"})),(0,r.kt)("p",null,"After saving the SAML config, you should see a ",(0,r.kt)("inlineCode",{parentName:"p"},"Sign in with Jumpcloud")," button\non your Firezone portal sign-in page."))}m.isMDXComponent=!0}}]);