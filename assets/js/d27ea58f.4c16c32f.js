"use strict";(self.webpackChunknew_docs=self.webpackChunknew_docs||[]).push([[4449],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>d});var r=n(7294);function o(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){o(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,o=function(e,t){if(null==e)return{};var n,r,o={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(o[n]=e[n]);return o}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(o[n]=e[n])}return o}var s=r.createContext({}),c=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},p=function(e){var t=c(e.components);return r.createElement(s.Provider,{value:t},e.children)},m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,o=e.mdxType,i=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),u=c(n),d=o,f=u["".concat(s,".").concat(d)]||u[d]||m[d]||i;return n?r.createElement(f,a(a({ref:t},p),{},{components:n})):r.createElement(f,a({ref:t},p))}));function d(e,t){var n=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var i=n.length,a=new Array(i);a[0]=u;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l.mdxType="string"==typeof e?e:o,a[1]=l;for(var c=2;c<i;c++)a[c]=n[c];return r.createElement.apply(null,a)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},9278:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>a,default:()=>m,frontMatter:()=>i,metadata:()=>l,toc:()=>c});var r=n(7462),o=(n(7294),n(3905));const i={title:"Telemetry",sidebar_position:4},a=void 0,l={unversionedId:"reference/telemetry",id:"reference/telemetry",title:"Telemetry",description:"This document presents an overview of the telemetry Firezone collects from your",source:"@site/docs/reference/telemetry.md",sourceDirName:"reference",slug:"/reference/telemetry",permalink:"/reference/telemetry",draft:!1,editUrl:"https://github.com/firezone/firezone/tree/master/docs/reference/telemetry.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{title:"Telemetry",sidebar_position:4},sidebar:"tutorialSidebar",previous:{title:"nftables Firewall Template",permalink:"/reference/firewall-templates/nftables"}},s={},c=[{value:"Why Firezone collects telemetry",id:"why-firezone-collects-telemetry",level:2},{value:"How we collect telemetry",id:"how-we-collect-telemetry",level:2},{value:"How to disable telemetry",id:"how-to-disable-telemetry",level:2}],p={toc:c};function m(e){let{components:t,...n}=e;return(0,o.kt)("wrapper",(0,r.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("p",null,"This document presents an overview of the telemetry Firezone collects from your\nself-hosted instance and how to disable it."),(0,o.kt)("h2",{id:"why-firezone-collects-telemetry"},"Why Firezone collects telemetry"),(0,o.kt)("p",null,"We ",(0,o.kt)("em",{parentName:"p"},"rely")," on telemetry to prioritize our roadmap and optimize the engineering\nresources we have to make Firezone better for everyone."),(0,o.kt)("p",null,"The telemetry we collect aims to answer the following questions:"),(0,o.kt)("ul",null,(0,o.kt)("li",{parentName:"ul"},"How many people install, use, and stop using Firezone?"),(0,o.kt)("li",{parentName:"ul"},"What features are most valuable, and which ones don\u2019t see any use?"),(0,o.kt)("li",{parentName:"ul"},"What functionality needs the most improvement?"),(0,o.kt)("li",{parentName:"ul"},"When something breaks, why did it break, and how can we prevent it from happening\nin the future?")),(0,o.kt)("h2",{id:"how-we-collect-telemetry"},"How we collect telemetry"),(0,o.kt)("p",null,"There are three main places where telemetry is collected in Firezone:"),(0,o.kt)("ol",null,(0,o.kt)("li",{parentName:"ol"},"Package telemetry. Includes events such as install, uninstall, and upgrade."),(0,o.kt)("li",{parentName:"ol"},"CLI telemetry from ",(0,o.kt)("inlineCode",{parentName:"li"},"firezone-ctl")," commands."),(0,o.kt)("li",{parentName:"ol"},"Product telemetry associated with the Web portal.")),(0,o.kt)("p",null,"In each of these three contexts, we capture the minimum amount of data necessary\nto answer the questions in the section above."),(0,o.kt)("p",null,"Admin emails are collected ",(0,o.kt)("strong",{parentName:"p"},"only if")," you explicitly opt-in to product updates.\nOtherwise, personally-identifiable information is ",(0,o.kt)("strong",{parentName:"p"},(0,o.kt)("em",{parentName:"strong"},"never"))," collected."),(0,o.kt)("p",null,"We store telemetry in a self-hosted instance of PostHog running in a private\nKubernetes cluster, only accessible by the Firezone team. Here is an example of\na telemetry event that is sent from your instance of Firezone to our telemetry server:"),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-json"},'{\n    "id": "0182272d-0b88-0000-d419-7b9a413713f1",\n    "timestamp": "2022-07-22T18:30:39.748000+00:00",\n    "event": "fz_http_started",\n    "distinct_id": "1ec2e794-1c3e-43fc-a78f-1db6d1a37f54",\n    "properties": {\n        "$geoip_city_name": "Ashburn",\n        "$geoip_continent_code": "NA",\n        "$geoip_continent_name": "North America",\n        "$geoip_country_code": "US",\n        "$geoip_country_name": "United States",\n        "$geoip_latitude": 39.0469,\n        "$geoip_longitude": -77.4903,\n        "$geoip_postal_code": "20149",\n        "$geoip_subdivision_1_code": "VA",\n        "$geoip_subdivision_1_name": "Virginia",\n        "$geoip_time_zone": "America/New_York",\n        "$ip": "52.200.241.107",\n        "$plugins_deferred": [],\n        "$plugins_failed": [],\n        "$plugins_succeeded": [\n            "GeoIP (3)"\n        ],\n        "distinct_id": "1zc2e794-1c3e-43fc-a78f-1db6d1a37f54",\n        "fqdn": "awsdemo.firezone.dev",\n        "kernel_version": "linux 5.13.0",\n        "version": "0.4.6"\n    },\n    "elements_chain": ""\n}\n')),(0,o.kt)("h2",{id:"how-to-disable-telemetry"},"How to disable telemetry"),(0,o.kt)("admonition",{type:"note"},(0,o.kt)("p",{parentName:"admonition"},"We ",(0,o.kt)("em",{parentName:"p"},"rely")," on product analytics to make Firezone better for everyone.\nLeaving telemetry enabled is the ",(0,o.kt)("strong",{parentName:"p"},"single most valuable contribution")," you can\nmake to Firezone\u2019s development. That said, we understand some users have higher\nprivacy or security requirements and would prefer to disable telemetry altogether.\nIf that\u2019s you, keep reading.")),(0,o.kt)("p",null,"Telemetry is enabled by default. To completely disable product telemetry, set the\nfollowing configuration option to ",(0,o.kt)("inlineCode",{parentName:"p"},"false")," in ",(0,o.kt)("inlineCode",{parentName:"p"},"/etc/firezone/firezone.rb")," and run\n",(0,o.kt)("inlineCode",{parentName:"p"},"sudo firezone-ctl reconfigure")," to pick up the changes."),(0,o.kt)("pre",null,(0,o.kt)("code",{parentName:"pre",className:"language-ruby"},"default['firezone']['telemetry']['enabled'] = false\n")),(0,o.kt)("p",null,"That will completely disable all product telemetry."),(0,o.kt)("admonition",{type:"note"},(0,o.kt)("p",{parentName:"admonition"},"If you\u2019re looking for support running Firezone in air-gapped or other restrictive\nenvironments, ",(0,o.kt)("a",{parentName:"p",href:"mailto:sales@firezone.dev"},"contact us")," about our\n",(0,o.kt)("a",{parentName:"p",href:"https://www.firezone.dev/pricing"},"Enterprise"),"\xa0functionality.")))}m.isMDXComponent=!0}}]);