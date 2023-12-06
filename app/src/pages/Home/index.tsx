import { PageContainer } from '@ant-design/pro-components';
import moreJson from '@/data/more.json';
import './index.less';

const HomePage: React.FC = () => {
  const {
    mako,
    webpack,
    mako_len,
    webpack_len
  } = moreJson || {};

  return (
    <PageContainer ghost>
      <div className='home-container'>
        <div className='item'>
          <h2>mako</h2>
          <h3>
            总模块数: {mako_len.length}
          </h3>
          <h3>差异模块列表</h3>
          {
            mako.map((str, index) => <div className="list-item" key={str}>{index + 1}: {str}</div>)
          }
        </div>
        <div className='item'>
          <h2>webpack</h2>
          <h3>
            总模块数: {webpack_len.length}
          </h3>
          <h3>差异模块列表</h3>
          {
            webpack.map((str, index) => <div className="list-item" key={str}>{index + 1}: {str}</div>)
          }
        </div>
      </div>
    </PageContainer>
  );
};

export default HomePage;
