"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[6444],{3905:(e,t,n)=>{n.d(t,{Zo:()=>s,kt:()=>h});var i=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function a(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);t&&(i=i.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,i)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?a(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):a(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,i,r=function(e,t){if(null==e)return{};var n,i,r={},a=Object.keys(e);for(i=0;i<a.length;i++)n=a[i],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(i=0;i<a.length;i++)n=a[i],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var u=i.createContext({}),c=function(e){var t=i.useContext(u),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},s=function(e){var t=c(e.components);return i.createElement(u.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return i.createElement(i.Fragment,{},t)}},d=i.forwardRef((function(e,t){var n=e.components,r=e.mdxType,a=e.originalType,u=e.parentName,s=l(e,["components","mdxType","originalType","parentName"]),d=c(n),h=r,f=d["".concat(u,".").concat(h)]||d[h]||p[h]||a;return n?i.createElement(f,o(o({ref:t},s),{},{components:n})):i.createElement(f,o({ref:t},s))}));function h(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var a=n.length,o=new Array(a);o[0]=d;var l={};for(var u in t)hasOwnProperty.call(t,u)&&(l[u]=t[u]);l.originalType=e,l.mdxType="string"==typeof e?e:r,o[1]=l;for(var c=2;c<a;c++)o[c]=n[c];return i.createElement.apply(null,o)}return i.createElement.apply(null,n)}d.displayName="MDXCreateElement"},2684:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>u,contentTitle:()=>o,default:()=>p,frontMatter:()=>a,metadata:()=>l,toc:()=>c});var i=n(7462),r=(n(7294),n(3905));const a={title:"Authenticate",sidebar_position:3},o=void 0,l={unversionedId:"authenticate/README",id:"authenticate/README",title:"Authenticate",description:"Firezone can be configured to require authentication before users can generate",source:"@site/docs/authenticate/README.md",sourceDirName:"authenticate",slug:"/authenticate/",permalink:"/authenticate/",draft:!1,editUrl:"https://github.com/firezone/firezone/tree/master/docs/authenticate/README.md",tags:[],version:"current",sidebarPosition:3,frontMatter:{title:"Authenticate",sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"HAProxy",permalink:"/deploy/reverse-proxies/haproxy"},next:{title:"Local Authentication",permalink:"/authenticate/local-auth"}},u={},c=[{value:"Integrate A SSO Provider",id:"integrate-a-sso-provider",level:2},{value:"The OIDC Redirect URL",id:"the-oidc-redirect-url",level:3},{value:"Enforce Periodic Re-authentication",id:"enforce-periodic-re-authentication",level:2},{value:"Re-authentication",id:"re-authentication",level:3},{value:"VPN Connection Status",id:"vpn-connection-status",level:4}],s={toc:c};function p(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,i.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"Firezone can be configured to require authentication before users can generate\nor download device configuration files. Optionally,\n",(0,r.kt)("a",{parentName:"p",href:"#enforce-periodic-re-authentication"},"periodic re-authentication"),"\ncan also be required for users to maintain their VPN session."),(0,r.kt)("p",null,"Firezone supports the following authentication methods:"),(0,r.kt)("ol",null,(0,r.kt)("li",{parentName:"ol"},(0,r.kt)("a",{parentName:"li",href:"../authenticate/local-auth"},"Local email/password"),": Enabled by default. A ",(0,r.kt)("a",{parentName:"li",href:"../authenticate/multi-factor"},"time-based\none time password (TOTP)"),"\ncan optionally be configured to add an additional authentication factor."),(0,r.kt)("li",{parentName:"ol"},(0,r.kt)("a",{parentName:"li",href:"#integrate-a-sso-provider"},"Single Sign-On (SSO)"),": Enables users to sign\nin using their credentials from Okta, Google, Azure AD, or any service supporting\nthe OpenID Connect (OIDC) protocol.")),(0,r.kt)("h2",{id:"integrate-a-sso-provider"},"Integrate A SSO Provider"),(0,r.kt)("p",null,"We've included instructions on how to set up Firezone with several popular\nidentity providers:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"../authenticate/okta"},"Okta")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"../authenticate/azuread"},"Azure Active Directory")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"../authenticate/google"},"Google")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("a",{parentName:"li",href:"../authenticate/onelogin"},"Onelogin"))),(0,r.kt)("p",null,"If your identity provider is not listed above, but has a generic OIDC\nconnector, please consult their documentation to find instructions on obtaining\nthe config settings required. Instructions on setting up Firezone with a generic\nOIDC provider can be found ",(0,r.kt)("a",{parentName:"p",href:"../authenticate/generic-oidc"},"here"),"."),(0,r.kt)("p",null,"Open a ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/firezone/firezone/issues"},"Github Issue"),"\nto request documentation\nor submit a ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/firezone/firezone/tree/master/docs/docs/authenticate/index.md"},"pull request"),"\nto add documentation for your provider.\nIf you require assistance in setting up your OIDC provider, please\njoin the ",(0,r.kt)("a",{parentName:"p",href:"https://www.firezone.dev/slack"},"Firezone Slack group"),"."),(0,r.kt)("h3",{id:"the-oidc-redirect-url"},"The OIDC Redirect URL"),(0,r.kt)("p",null,"For each OIDC provider a corresponding URL is created for redirecting to\nthe configured provider's sign-in URL. The URL format is ",(0,r.kt)("inlineCode",{parentName:"p"},"https://firezone.example.com/auth/oidc/PROVIDER"),"\nwhere ",(0,r.kt)("inlineCode",{parentName:"p"},"PROVIDER")," is the OIDC key for that particular provider."),(0,r.kt)("p",null,"For example, the OIDC config below"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-ruby"},"default['firezone']['authentication']['oidc'] = {\ngoogle: {\n  # ...\n},\nokta: {\n  # ...\n}\n")),(0,r.kt)("p",null,"would generate the following URLs:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"https://firezone.example.com/auth/oidc/google")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"https://firezone.example.com/auth/oidc/okta"))),(0,r.kt)("p",null,"These URLs could then be distributed by an Admin directly to end users to navigate\nto the appropriate identity provider login page to authenticate to Firezone."),(0,r.kt)("h2",{id:"enforce-periodic-re-authentication"},"Enforce Periodic Re-authentication"),(0,r.kt)("p",null,"Periodic re-authentication can be enforced by changing the setting in\n",(0,r.kt)("inlineCode",{parentName:"p"},"settings/security"),". This can be used to ensure a user must sign in to Firezone\nperiodically in order to maintain their VPN session."),(0,r.kt)("p",null,"You can set the session length to a minimum of ",(0,r.kt)("strong",{parentName:"p"},"1 hour")," and maximum of ",(0,r.kt)("strong",{parentName:"p"},"90 days"),".\nSetting this to Never disables this setting, allowing VPN sessions indefinitely.\nThis is the default."),(0,r.kt)("h3",{id:"re-authentication"},"Re-authentication"),(0,r.kt)("p",null,"To re-authenticate an expired VPN session, a user will need to turn off their\nVPN session and sign in to the Firezone portal (URL specified during\n",(0,r.kt)("a",{parentName:"p",href:"../deploy/prerequisites"},"deployment"),"\n)."),(0,r.kt)("p",null,"See detailed Client Instructions on how to re-authenticate your session\n",(0,r.kt)("a",{parentName:"p",href:"../user-guides/client-instructions"},"here"),"."),(0,r.kt)("h4",{id:"vpn-connection-status"},"VPN Connection Status"),(0,r.kt)("p",null,"A user's connection status is shown on the Users page under the table column\n",(0,r.kt)("inlineCode",{parentName:"p"},"VPN Connection"),". The connection statuses are:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"ENABLED - The connection is enabled."),(0,r.kt)("li",{parentName:"ul"},"DISABLED - The connection is disabled by an administrator or OIDC refresh failure."),(0,r.kt)("li",{parentName:"ul"},"EXPIRED - The connection is disabled due to authentication expiration or a user\nhas not signed in for the first time.")))}p.isMDXComponent=!0}}]);