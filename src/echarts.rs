/// Echart module 
/// 
/// # TODO move to server-api
/// 
/// * [ ] appending data on fly https://www.echartsjs.com/examples/en/editor.html?c=linesGL-ny&gl=1
/// 
/// ```
/// myChart.appendData({
///   seriesIndex: 0,
///   data: data
/// });
/// 
/// Can use
///
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use serde::{Serialize, Deserialize};

#[wasm_bindgen(raw_module = "/html/js/echart-line.js")]
extern "C" {
    pub fn show_chart(class_id: JsValue, x_data: js_sys::Array, y_data: js_sys::Array);
    pub fn show_chart_with_series(class_id: JsValue, x_data: Vec<JsValue>, data_series: Vec<JsValue>);
    pub fn show_chart_with_option(class_id: JsValue, option: JsValue);
}


pub struct Chart {
    pub id: String,
    pub opt: Opt,
}

impl Chart {
    pub fn new<T: Into<String>>(id: T) -> Self {
        Chart { id: id.into(), opt: Opt::default() }
    }
    pub fn show(&self) {
        let class_id = JsValue::from(&self.id);
        let opt = JsValue::from_serde(&self.opt).unwrap();
        show_chart_with_option(class_id, opt);
    }

    pub fn add_x_data(&mut self, data: Vec<String>) {
        self.opt.x_axis.data = data;
    }

    pub fn add_y_data(&mut self, name: String, r#type: String, data: Vec<f32>) {
        let serie = Serie {
            name,
            data,
            r#type,
            ..Default::default()
        };
        self.opt.series.push(serie);
    }
}


#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Opt {
    tooltip: ToolTip,
    pub data_zoom: Vec<DataZoom>,
    pub legend: Legend,
    x_axis: XAxis,
    y_axis: YAxis,
    series: Vec<Serie>,
    animation: bool,
}


// #[wasm_bindgen]
#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Serie {
    pub name: String,
    pub data: Vec<f32>, 
    pub r#type: String,
    pub smooth: Option<bool>,
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    pub data: Vec<String>
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolTip {
    trigger: String,
    axis_pointer: AxisPointer,
}

impl Default for ToolTip {
    fn default() -> Self {
        ToolTip {
            trigger: "axis".to_string(),
            axis_pointer: AxisPointer {
                animation: false,
            }
        }
    }
}

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer {
    pub animation: bool,
}


#[derive(Serialize, Default, Clone)]
pub struct XAxis {
    #[serde(default = "category")]
    r#type: String,
    boundary_gap: bool,
    data: Vec<String>,
}

#[derive(Serialize, Default, Clone)]
pub struct YAxis {
    #[serde(default = "value")]
    r#type: String,
}


#[derive(Serialize, Clone)]
pub struct DataZoom {
    pub show: bool,
    pub realtime: bool,
    pub start: i32,
    pub end: i32,
    pub x_axis_index: i32,
}

impl Default for DataZoom {
    fn default() -> Self {
        DataZoom {
            show: true,
            realtime: true,
            start: 0,
            end: 100,
            x_axis_index: 0,
        }
    }
}

