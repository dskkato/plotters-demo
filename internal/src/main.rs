use image::imageops::FilterType;
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use std::error::Error;

const OUT_FILE_NAME: &str = "sample.png";

fn main() -> Result<(), Box<dyn Error>> {
    let (width, height) = (400, 300);
    let mut canvas = image::RgbImage::new(width, height);

    let x: Vec<i32> = (0..10).collect();
    let y: Vec<f32> = x.iter().map(|x| (x * x) as f32).collect();
    let points: Vec<_> = x
        .iter()
        .zip(y.iter())
        .map(|(&v0, &v1)| (v0 as f32, v1 as f32))
        .collect();

    // limit `buf` life time
    {
        // use plotters::coord::ranged1d::types::RangedCoordf32;
        let mut buf = canvas.as_flat_samples_mut();

        let root =
            BitMapBackend::<RGBPixel>::with_buffer_and_format(buf.as_mut_slice(), (width, height))?
                .into_drawing_area();
        root.fill(&WHITE)?;

        // ラベル文字が切れないように適当なmarginを設定しておく
        // marginの大きさは対象の描画領域に依存することに注意
        let root = root.margin(20, 20, 20, 20);

        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("Arial", 20).into_font())
            .x_label_area_size(40) // x軸ラベルの表示領域を用意しておく。
            .y_label_area_size(40) // y軸ラベルの表示領域を用意しておく。
            .build_cartesian_2d(-0.5f32..9.5f32, -5f32..85f32)?;

        // plot_areaの一部に表示させる場合には、表示エリアと目的の表示範囲の情報を使って、
        // 明示的にリサイズする必要がある。
        let dst_w = 8.0f32;
        let dst_h = 80.0f32;
        let x_range = chart.x_range();
        let x_scale = dst_w / (x_range.end - x_range.start);
        let y_range = chart.y_range();
        let y_scale = dst_h / (y_range.end - y_range.start);

        // plot areaのピクセル数を取得する
        let (w, h) = chart.plotting_area().dim_in_pixel();

        // plot areaの一部に表示させるためのサイズを計算する。
        let w = (w as f32 * x_scale) as u32;
        let h = (h as f32 * y_scale) as u32;
        // 明示的にリサイズする
        let img = image::open("../macaque.jpg")?.resize_exact(w, h, FilterType::Gaussian);

        // 描画
        let elem: BitMapElement<_> = ((0.0, 80.0), img).into(); // or
                                                                // let elem = BitMapElement::<_>::from(((0.0f32, 80.0f32), img));

        chart.draw_series(std::iter::once(elem))?;

        chart.draw_series(LineSeries::new(points.iter().copied(), &BLUE))?;

        chart.draw_series(
            points
                .iter()
                .map(|v| Circle::new((v.0, v.1), 3, BLUE.filled())),
        )?;

        chart
            .configure_mesh()
            .light_line_style(BLACK.stroke_width(0)) // stroke width 0にminor tickを消す
            .draw()?; // draw ticks
                      // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    }

    canvas.save(OUT_FILE_NAME)?;
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}
