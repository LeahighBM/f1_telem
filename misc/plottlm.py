import re
import plotly.graph_objects as go
from plotly.subplots import make_subplots

def parse_telemetry_file(filename):
    data = []

    with open(filename, 'r') as file:
        for line in file:
            match = re.search(r'CarTelemetryPacket\s*{(.*)}', line)
            if match:
                values_str = match.group(1)
                fields = {}
                for item in values_str.split(','):
                    if ':' in item:
                        key, val = item.split(':', 1)
                        key = key.strip()
                        val = val.strip()
                        try:
                            fields[key] = float(val) if '.' in val or 'e' in val.lower() else int(val)
                        except ValueError:
                            fields[key] = val
                data.append(fields)
    return data

def plot_dashboard(data, fields_to_plot, output_file='telemetry_dashboard.html'):
    from math import ceil

    x = list(range(len(data)))
    total = len(fields_to_plot)
    rows = ceil(total / 2)
    cols = 2 if total > 1 else 1

    fig = make_subplots(rows=rows, cols=cols, subplot_titles=fields_to_plot)

    for idx, field in enumerate(fields_to_plot):
        row = idx // cols + 1
        col = idx % cols + 1
        y = [entry[field] for entry in data]
        fig.add_trace(
            go.Scatter(x=x, y=y, mode='lines', name=field),
            row=row, col=col
        )

    fig.update_layout(
        title_text="Car Telemetry Dashboard",
        height=300 * rows,
        showlegend=False,
        template="plotly_white"
    )

    fig.write_html(output_file, include_plotlyjs='cdn')
    print(f"Dashboard saved to {output_file}")

if __name__ == "__main__":
    filename = "sample_car_data.txt"
    fields = ['speed_kph', 'throttle', 'steer', 'brake', 'gear', 'eng_rpm', 'drs', 'engine_temp']
    telemetry_data = parse_telemetry_file(filename)
    plot_dashboard(telemetry_data, fields)
