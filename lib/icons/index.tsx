import { ref, watch, defineComponent } from 'vue'
import './index.less'
// @ts-ignore
import defaultFileUrl from '/monaco-tree-editor-statics/icons/default_file.svg?url'
// @ts-ignore
import javaUrl from '/monaco-tree-editor-statics/icons/file_type_java.svg?url'
// @ts-ignore
import kotlinUrl from '/monaco-tree-editor-statics/icons/file_type_kotlin.svg?url'
// @ts-ignore
import pythonUrl from '/monaco-tree-editor-statics/icons/file_type_python.svg?url'// @ts-ignore
import cUrl from '/monaco-tree-editor-statics/icons/file_type_c.svg?url'// @ts-ignore
import cheaderUrl from '/monaco-tree-editor-statics/icons/file_type_cheader.svg?url'// @ts-ignore
import cppUrl from '/monaco-tree-editor-statics/icons/file_type_cpp.svg?url'// @ts-ignore
import csharpUrl from '/monaco-tree-editor-statics/icons/file_type_csharp.svg?url'// @ts-ignore
import rustUrl from '/monaco-tree-editor-statics/icons/file_type_rust.svg?url'// @ts-ignore
import vueUrl from '/monaco-tree-editor-statics/icons/file_type_vue.svg?url'// @ts-ignore
import batUrl from '/monaco-tree-editor-statics/icons/file_type_bat.svg?url'// @ts-ignore
import shellUrl from '/monaco-tree-editor-statics/icons/file_type_shell.svg?url'// @ts-ignore
import luaUrl from '/monaco-tree-editor-statics/icons/file_type_lua.svg?url'// @ts-ignore
import jsUrl from '/monaco-tree-editor-statics/icons/file_type_js.svg?url'// @ts-ignore
import tsUrl from '/monaco-tree-editor-statics/icons/file_type_typescript.svg?url'// @ts-ignore
import tsxUrl from '/monaco-tree-editor-statics/icons/file_type_reactts.svg?url'// @ts-ignore
import jsxUrl from '/monaco-tree-editor-statics/icons/file_type_reactjs.svg?url'// @ts-ignore
import htmlUrl from '/monaco-tree-editor-statics/icons/file_type_html.svg?url'// @ts-ignore
import cssUrl from '/monaco-tree-editor-statics/icons/file_type_css.svg?url'// @ts-ignore
import lessUrl from '/monaco-tree-editor-statics/icons/file_type_less.svg?url'// @ts-ignore
import jsonUrl from '/monaco-tree-editor-statics/icons/file_type_json.svg?url'// @ts-ignore
import json5Url from '/monaco-tree-editor-statics/icons/file_type_json5.svg?url'// @ts-ignore
import svgUrl from '/monaco-tree-editor-statics/icons/file_type_svg.svg?url'// @ts-ignore
import dockerUrl from '/monaco-tree-editor-statics/icons/file_type_docker.svg?url'// @ts-ignore
import sqlUrl from '/monaco-tree-editor-statics/icons/file_type_sql.svg?url'// @ts-ignore
import tomlUrl from '/monaco-tree-editor-statics/icons/file_type_toml.svg?url'// @ts-ignore
import xmlUrl from '/monaco-tree-editor-statics/icons/file_type_xml.svg?url'// @ts-ignore
import yamlUrl from '/monaco-tree-editor-statics/icons/file_type_yaml.svg?url'// @ts-ignore
import markdownUrl from '/monaco-tree-editor-statics/icons/file_type_markdown.svg?url'// @ts-ignore
import git2Url from '/monaco-tree-editor-statics/icons/file_type_git2.svg?url'// @ts-ignore
import iniUrl from '/monaco-tree-editor-statics/icons/file_type_ini.svg?url'// @ts-ignore
import gradleUrl from '/monaco-tree-editor-statics/icons/file_type_gradle.svg?url'// @ts-ignore
import httpUrl from '/monaco-tree-editor-statics/icons/file_type_http.svg?url'// @ts-ignore
import dbUrl from '/monaco-tree-editor-statics/icons/file_type_db.svg?url'

export default defineComponent({
  props: {
    type: {
      type: String,
      default: 'default_file',
    },
  },
  setup(props) {
    const fileTypeMap = (fileType: string): string => {
      const type = fileType.split('_').slice(-1)[0]
      const config: {
        [index: string]: string
      } = {
        java: javaUrl,
        kt: kotlinUrl,
        kts: kotlinUrl,
        py: pythonUrl,
        c: cUrl,
        h: cheaderUrl,
        cpp: cppUrl,
        cs: csharpUrl,
        rs: rustUrl,
        vue: vueUrl,
        bat: batUrl,
        sh: shellUrl,
        lua: luaUrl,
        ts: tsUrl,
        js: jsUrl,
        tsx: tsxUrl,
        jsx: jsxUrl,
        html: htmlUrl,
        css: cssUrl,
        less: lessUrl,
        json: jsonUrl,
        json5: json5Url,
        svg: svgUrl,
        docker: dockerUrl,
        sql: sqlUrl,
        toml: tomlUrl,
        xml: xmlUrl,
        yaml: yamlUrl,
        yml: yamlUrl,
        md: markdownUrl,
        markdown: markdownUrl,
        gitignore: git2Url,
        ini: iniUrl,
        gradle: gradleUrl,
        http: httpUrl,
        db: dbUrl,
      }
      return config[type] ? config[type] : fileType
    }
    const imgSrc = ref(fileTypeMap(props.type))
    const handleError = () => {
      imgSrc.value = defaultFileUrl
    }
    watch(
      () => props.type,
      (n) => {
        imgSrc.value = fileTypeMap(n)
      }
    )
    return () => {
      return <img src={imgSrc.value} onError={handleError} class="music-monaco-icons" />
    }
  },
})
