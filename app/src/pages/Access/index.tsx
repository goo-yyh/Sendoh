import { PageContainer } from '@ant-design/pro-components';
import DiffJson from '@/data/diff.json';
import { Button, Select } from 'antd';
import { useState } from 'react';
import './index.less';

const LIMIT = 50;

const AccessPage: React.FC = () => {
  const [ind, setInd] = useState(0);

  const { diff } = DiffJson;

  const list = diff.slice(0, 50);

  const options = list.map((item, index) => {
    const { webpack, diff_line } = item;
    return {
      label: `(差异${diff_line}行) ${webpack}`,
      value: index
    }
  })

  const selectedItem = list[ind || 0];

  const {
    webpack_line,
    mako_line,
    webpack_code,
    mako_code,
    diff_line
  } = selectedItem;


  return (
    <PageContainer
      ghost
      header={{
        title: '代码差异分析',
      }}
    >
      <div>
        <Select
          value={ind}
          style={{ width: '100%' }}
          placeholder="请选择需要对比的模块"
          options={options}
          onChange={val => setInd(val)}
        />
      </div>
      {
        !selectedItem ? null : <div className="diff-container">
          <div className="item">
            <h2>mako</h2>
            <div>
              {mako_line}行，
              {mako_line > webpack_line ? '多于' : '少于'} webpack {diff_line} 行
            </div>
            <div className="code-container">
              {mako_code}
            </div>
          </div>
          <div className="item">
            <h2>webpack</h2>
            <div>
              {webpack_line}行
            </div>
            <div className="code-container">
              {webpack_code}
            </div>
          </div>
        </div>
      }
    </PageContainer>
  );
};

export default AccessPage;
